use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use axum::{
    Json,
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use tokio::process::Command;

const MAX_SUPPLY: u64 = 100;
const COOLDOWN_SECONDS: u64 = 5 * 60;
const PRICE_USDC_MICRO: &str = "1000000"; // $1.00 USDC with 6 decimals
const BASE_USDC: &str = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913";
const AGENTCASH_BASE_TREASURY: &str = "0x73731620Bd6bB9EDc3c77E5982665fCBA797D710";

#[derive(Clone)]
pub struct ShopState {
    inner: Arc<Mutex<InnerShopState>>,
    state_path: Option<PathBuf>,
}

#[derive(Default, Serialize, Deserialize)]
struct InnerShopState {
    minted: u64,
    last_successful_mint_at: Option<u64>,
    mints: Vec<MintRecord>,
    #[serde(default)]
    settled_payment_headers: HashSet<String>,
}

impl Default for ShopState {
    fn default() -> Self {
        Self::from_env()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MintRecord {
    token_id: u64,
    wallet: String,
    prompt: String,
    token_uri: String,
    minted_at: u64,
    tx_hash: Option<String>,
}

#[derive(Serialize)]
pub struct ShopStatus {
    minted: u64,
    max_supply: u64,
    price_usd: u64,
    cooldown_seconds: u64,
    network: &'static str,
    chain_id: u64,
    pay_to: String,
    asset: &'static str,
    latest: Vec<MintRecord>,
}

#[derive(Deserialize)]
pub struct MintRequest {
    wallet: String,
    prompt: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MintResponse {
    token_id: u64,
    wallet: String,
    prompt: String,
    token_uri: String,
    minted_at: u64,
    remaining_supply: u64,
    tx_hash: Option<String>,
    contract_address: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PaymentRequired {
    x402_version: u8,
    error: &'static str,
    resource: X402Resource,
    accepts: Vec<X402Accept>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct X402Resource {
    url: String,
    description: &'static str,
    mime_type: &'static str,
    service_name: &'static str,
    tags: Vec<&'static str>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct X402Accept {
    scheme: &'static str,
    network: &'static str,
    amount: &'static str,
    pay_to: String,
    max_timeout_seconds: u64,
    asset: &'static str,
    extra: serde_json::Value,
}

#[derive(Debug)]
struct ShopConfig {
    public_base_url: String,
    treasury_address: String,
    x402_facilitator_url: String,
    ipfs_pinata_jwt: Option<String>,
    nft_contract_address: Option<String>,
    mint_command: String,
    fulfillment_mode: FulfillmentMode,
    state_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FulfillmentMode {
    Local,
    Onchain,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct X402SettleResponse {
    success: bool,
    #[serde(default)]
    transaction: Option<String>,
    #[serde(default)]
    error_reason: Option<String>,
    #[serde(default)]
    error_message: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChainMintResult {
    tx_hash: String,
    token_id: u64,
    total_supply: u64,
    last_minted_at: u64,
    contract_address: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PinataResponsePascal {
    ipfs_hash: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PinataResponseCamel {
    ipfs_hash: Option<String>,
    cid: Option<String>,
}

#[derive(Serialize)]
struct NftAttribute<'a> {
    trait_type: &'a str,
    value: String,
}

#[derive(Serialize)]
struct NftMetadata<'a> {
    name: String,
    description: &'a str,
    image: String,
    attributes: Vec<NftAttribute<'a>>,
}

pub async fn status(State(state): State<ShopState>) -> Json<ShopStatus> {
    Json(state.status())
}

pub async fn mint(
    State(state): State<ShopState>,
    headers: HeaderMap,
    Json(payload): Json<MintRequest>,
) -> impl IntoResponse {
    let payment_signature = headers
        .get("payment-signature")
        .or_else(|| headers.get("x-payment"));

    if payment_signature.is_none() {
        let requirements = payment_requirements();
        let encoded_requirements = encode_payment_required_header(&requirements);
        let mut response = (
            StatusCode::PAYMENT_REQUIRED,
            Json(serde_json::to_value(requirements).expect("payment json")),
        )
            .into_response();
        response.headers_mut().insert(
            "PAYMENT-REQUIRED",
            HeaderValue::from_str(&encoded_requirements).expect("base64 payment requirements"),
        );
        return response;
    }

    let payment_header = match payment_signature.and_then(|value| value.to_str().ok()) {
        Some(value) if !value.trim().is_empty() => value.trim().to_owned(),
        _ => {
            return (
                StatusCode::PAYMENT_REQUIRED,
                Json(serde_json::json!({ "error": "valid PAYMENT-SIGNATURE header required" })),
            )
                .into_response();
        }
    };

    match state.mint_paid(payload, payment_header).await {
        Ok((response, payment_response_header)) => {
            let mut response = (
                StatusCode::OK,
                Json(serde_json::to_value(response).expect("mint json")),
            )
                .into_response();
            response.headers_mut().insert(
                "PAYMENT-RESPONSE",
                HeaderValue::from_str(&payment_response_header).expect("base64 payment response"),
            );
            response
        }
        Err((status, message)) => {
            (status, Json(serde_json::json!({ "error": message }))).into_response()
        }
    }
}

fn load_inner_state(path: &Path) -> Option<InnerShopState> {
    let bytes = fs::read(path).ok()?;
    serde_json::from_slice(&bytes).ok()
}

impl ShopState {
    fn from_env() -> Self {
        match ShopConfig::from_env_for_paid_mint() {
            Ok(config) => Self::with_optional_path(config.state_path),
            Err(_) => Self::with_optional_path(None),
        }
    }

    #[cfg(test)]
    fn with_state_path(path: impl Into<PathBuf>) -> Self {
        Self::with_optional_path(Some(path.into()))
    }

    fn with_optional_path(state_path: Option<PathBuf>) -> Self {
        let inner = state_path
            .as_deref()
            .and_then(load_inner_state)
            .unwrap_or_default();
        Self {
            inner: Arc::new(Mutex::new(inner)),
            state_path,
        }
    }

    fn persist_locked(&self, inner: &InnerShopState) -> Result<(), (StatusCode, String)> {
        let Some(path) = &self.state_path else {
            return Ok(());
        };
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("failed to create shop state directory: {error}"),
                )
            })?;
        }
        let bytes = serde_json::to_vec_pretty(inner).map_err(|error| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to serialize shop state: {error}"),
            )
        })?;
        fs::write(path, bytes).map_err(|error| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to persist shop state: {error}"),
            )
        })
    }

    fn reserve_payment_header(&self, payment_header: &str) -> Result<(), (StatusCode, String)> {
        let mut inner = self.inner.lock().expect("shop state lock");
        if !inner
            .settled_payment_headers
            .insert(payment_header.to_owned())
        {
            return Err((
                StatusCode::CONFLICT,
                "x402 payment was already consumed for a mint".to_owned(),
            ));
        }
        self.persist_locked(&inner)
    }

    fn release_payment_header(&self, payment_header: &str) {
        let mut inner = self.inner.lock().expect("shop state lock");
        inner.settled_payment_headers.remove(payment_header);
        let _ = self.persist_locked(&inner);
    }

    fn status(&self) -> ShopStatus {
        let inner = self.inner.lock().expect("shop state lock");
        let mut latest = inner.mints.clone();
        latest.reverse();
        latest.truncate(6);

        ShopStatus {
            minted: inner.minted,
            max_supply: MAX_SUPPLY,
            price_usd: 1,
            cooldown_seconds: COOLDOWN_SECONDS,
            network: "base",
            chain_id: 8453,
            pay_to: treasury_address(),
            asset: BASE_USDC,
            latest,
        }
    }

    async fn mint_paid(
        &self,
        payload: MintRequest,
        payment_header: String,
    ) -> Result<(MintResponse, String), (StatusCode, String)> {
        let config = ShopConfig::from_env_for_paid_mint().map_err(|message| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                format!("shop launch configuration incomplete: {message}"),
            )
        })?;

        let wallet = validate_wallet(&payload.wallet)?;
        let prompt = payload
            .prompt
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "farmer cat tending a tiny lantern shop".to_owned());

        self.reserve_payment_header(&payment_header)?;
        self.ensure_can_start_mint().inspect_err(|_| {
            self.release_payment_header(&payment_header);
        })?;
        let settlement = match settle_x402_payment(&config, &payment_header).await {
            Ok(settlement) => settlement,
            Err(error) => {
                self.release_payment_header(&payment_header);
                return Err(error);
            }
        };
        let payment_response_header = encode_payment_response_header(&settlement);

        let next_token_id = self.next_token_id()?;
        let svg = generate_pixel_shop_svg(next_token_id, &prompt);
        let result = match config.fulfillment_mode {
            FulfillmentMode::Local => {
                let token_uri = format!(
                    "data:application/json;base64,{}",
                    STANDARD.encode(
                        serde_json::to_string(&local_metadata(next_token_id, &prompt, &svg))
                            .expect("local metadata json")
                    )
                );
                self.record_successful_mint(
                    wallet,
                    prompt,
                    token_uri,
                    settlement.transaction,
                    next_token_id,
                    next_token_id,
                    unix_now(),
                    std::env::var("SHOP_NFT_CONTRACT_ADDRESS").unwrap_or_default(),
                )
            }
            FulfillmentMode::Onchain => {
                let token_uri = pin_art_and_metadata(&config, next_token_id, &prompt, &svg).await?;
                let chain = mint_on_base(&config, &wallet, &token_uri).await?;
                self.record_successful_mint(
                    wallet,
                    prompt,
                    token_uri,
                    Some(chain.tx_hash),
                    chain.token_id,
                    chain.total_supply,
                    chain.last_minted_at,
                    chain.contract_address,
                )
            }
        };
        result.map(|response| (response, payment_response_header))
    }

    fn ensure_can_start_mint(&self) -> Result<(), (StatusCode, String)> {
        let inner = self.inner.lock().expect("shop state lock");
        if inner.minted >= MAX_SUPPLY {
            return Err((
                StatusCode::CONFLICT,
                "shop sold out: max 100 NFTs".to_owned(),
            ));
        }
        if let Some(last_mint) = inner.last_successful_mint_at {
            let elapsed = unix_now().saturating_sub(last_mint);
            if elapsed < COOLDOWN_SECONDS {
                return Err((
                    StatusCode::TOO_MANY_REQUESTS,
                    format!(
                        "endpoint cooldown active: wait {} more seconds before the next paid mint",
                        COOLDOWN_SECONDS - elapsed
                    ),
                ));
            }
        }
        Ok(())
    }

    fn next_token_id(&self) -> Result<u64, (StatusCode, String)> {
        let inner = self.inner.lock().expect("shop state lock");
        if inner.minted >= MAX_SUPPLY {
            return Err((
                StatusCode::CONFLICT,
                "shop sold out: max 100 NFTs".to_owned(),
            ));
        }
        Ok(inner.minted + 1)
    }

    fn record_successful_mint(
        &self,
        wallet: String,
        prompt: String,
        token_uri: String,
        tx_hash: Option<String>,
        token_id: u64,
        total_supply: u64,
        now: u64,
        contract_address: String,
    ) -> Result<MintResponse, (StatusCode, String)> {
        if !token_uri.starts_with("ipfs://")
            && !token_uri.starts_with("data:application/json;base64,")
        {
            return Err((
                StatusCode::BAD_GATEWAY,
                "IPFS pinning did not return an ipfs:// token URI".to_owned(),
            ));
        }

        let mut inner = self.inner.lock().expect("shop state lock");
        if inner.minted >= MAX_SUPPLY {
            return Err((
                StatusCode::CONFLICT,
                "shop sold out: max 100 NFTs".to_owned(),
            ));
        }
        if let Some(last_mint) = inner.last_successful_mint_at {
            let elapsed = now.saturating_sub(last_mint);
            if elapsed < COOLDOWN_SECONDS {
                return Err((
                    StatusCode::TOO_MANY_REQUESTS,
                    format!(
                        "endpoint cooldown active: wait {} more seconds before the next paid mint",
                        COOLDOWN_SECONDS - elapsed
                    ),
                ));
            }
        }

        inner.minted = total_supply;
        inner.last_successful_mint_at = Some(now);
        inner.mints.push(MintRecord {
            token_id,
            wallet: wallet.clone(),
            prompt: prompt.clone(),
            token_uri: token_uri.clone(),
            minted_at: now,
            tx_hash: tx_hash.clone(),
        });
        self.persist_locked(&inner)?;

        Ok(MintResponse {
            token_id,
            wallet,
            prompt,
            token_uri,
            minted_at: now,
            remaining_supply: MAX_SUPPLY.saturating_sub(total_supply),
            tx_hash,
            contract_address,
        })
    }

    #[cfg(test)]
    fn record_successful_mint_for_test(
        &self,
        wallet: String,
        prompt: String,
        token_uri: String,
        now: u64,
    ) -> Result<MintResponse, (StatusCode, String)> {
        self.record_successful_mint(
            wallet,
            prompt,
            token_uri,
            Some("0xtest".to_owned()),
            1,
            1,
            now,
            "0xcontract".to_owned(),
        )
    }
}

impl ShopConfig {
    fn from_env_for_paid_mint() -> Result<Self, String> {
        let public_base_url =
            std::env::var("PUBLIC_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".to_owned());
        let treasury_address = treasury_address();
        let x402_facilitator_url = std::env::var("X402_FACILITATOR_URL")
            .or_else(|_| {
                std::env::var("X402_SETTLE_URL")
                    .map(|url| url.trim_end_matches("/settle").to_owned())
            })
            .unwrap_or_else(|_| "https://x402.org/facilitator".to_owned());
        let fulfillment_mode = match std::env::var("SHOP_FULFILLMENT_MODE")
            .unwrap_or_else(|_| "local".to_owned())
            .to_lowercase()
            .as_str()
        {
            "local" | "development" => FulfillmentMode::Local,
            _ => FulfillmentMode::Onchain,
        };
        let ipfs_pinata_jwt = std::env::var("IPFS_PINATA_JWT")
            .ok()
            .filter(|value| !value.trim().is_empty());
        let nft_contract_address = std::env::var("SHOP_NFT_CONTRACT_ADDRESS")
            .ok()
            .filter(|value| !value.trim().is_empty());

        if fulfillment_mode == FulfillmentMode::Onchain {
            let mut missing = Vec::new();
            if ipfs_pinata_jwt.is_none() {
                missing.push("IPFS_PINATA_JWT");
            }
            if nft_contract_address.is_none() {
                missing.push("SHOP_NFT_CONTRACT_ADDRESS");
            }
            if !missing.is_empty() {
                return Err(format!("missing {}", missing.join(", ")));
            }
        }

        let mint_command = std::env::var("SHOP_MINT_COMMAND")
            .unwrap_or_else(|_| "node scripts/mint-farmer-token.mjs".to_owned());
        let state_path = std::env::var("SHOP_STATE_PATH")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .map(PathBuf::from);

        Ok(Self {
            public_base_url,
            treasury_address,
            x402_facilitator_url,
            ipfs_pinata_jwt,
            nft_contract_address,
            mint_command,
            fulfillment_mode,
            state_path,
        })
    }
}

fn payment_requirements() -> PaymentRequired {
    let host =
        std::env::var("PUBLIC_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".to_owned());
    let resource_url = format!("{host}/api/shop/mint");
    PaymentRequired {
        x402_version: 2,
        error: "PAYMENT-SIGNATURE header required. AgentCash can pay this x402 v2 route on Base before the shop generates the mint art.",
        resource: X402Resource {
            url: resource_url,
            description: "Mint one 0xFarmer 8-bit Japanese shop NFT",
            mime_type: "application/json",
            service_name: "0xFarmer Agent Shop",
            tags: vec!["agentcash", "x402", "base", "nft"],
        },
        accepts: vec![X402Accept {
            scheme: "exact",
            network: "eip155:8453",
            amount: PRICE_USDC_MICRO,
            pay_to: treasury_address(),
            max_timeout_seconds: 300,
            asset: BASE_USDC,
            extra: serde_json::json!({}),
        }],
    }
}

fn treasury_address() -> String {
    std::env::var("SHOP_TREASURY_ADDRESS").unwrap_or_else(|_| AGENTCASH_BASE_TREASURY.to_owned())
}

fn encode_payment_required_header(requirements: &PaymentRequired) -> String {
    STANDARD.encode(serde_json::to_string(requirements).expect("serialize payment requirements"))
}

fn encode_payment_response_header(response: &X402SettleResponse) -> String {
    STANDARD.encode(serde_json::to_string(response).expect("serialize payment response"))
}

async fn settle_x402_payment(
    config: &ShopConfig,
    payment_header: &str,
) -> Result<X402SettleResponse, (StatusCode, String)> {
    let payment_payload = decode_payment_signature(payment_header)?;
    let requirements = payment_requirements();
    let accepted = requirements.accepts.first().cloned().ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "no payment requirements configured".to_owned(),
        )
    })?;
    let client = reqwest::Client::new();
    let settle_url = format!(
        "{}/settle",
        config.x402_facilitator_url.trim_end_matches('/')
    );
    let response = client
        .post(settle_url)
        .json(&serde_json::json!({
            "x402Version": 2,
            "paymentPayload": payment_payload,
            "paymentRequirements": accepted,
        }))
        .send()
        .await
        .map_err(|error| {
            (
                StatusCode::BAD_GATEWAY,
                format!("x402 settlement request failed: {error}"),
            )
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err((
            StatusCode::PAYMENT_REQUIRED,
            format!("x402 payment settlement failed ({status}): {text}"),
        ));
    }

    let settled = response
        .json::<X402SettleResponse>()
        .await
        .map_err(|error| {
            (
                StatusCode::BAD_GATEWAY,
                format!("x402 settlement response was not JSON: {error}"),
            )
        })?;

    require_successful_settlement(settled)
}

fn require_successful_settlement(
    settled: X402SettleResponse,
) -> Result<X402SettleResponse, (StatusCode, String)> {
    if settled.success {
        return Ok(settled);
    }
    Err((
        StatusCode::PAYMENT_REQUIRED,
        format!(
            "x402 payment was rejected by facilitator: {} {}",
            settled.error_reason.clone().unwrap_or_default(),
            settled.error_message.clone().unwrap_or_default()
        ),
    ))
}

fn decode_payment_signature(
    payment_header: &str,
) -> Result<serde_json::Value, (StatusCode, String)> {
    let decoded = STANDARD.decode(payment_header).map_err(|error| {
        (
            StatusCode::PAYMENT_REQUIRED,
            format!("PAYMENT-SIGNATURE was not valid base64: {error}"),
        )
    })?;
    serde_json::from_slice(&decoded).map_err(|error| {
        (
            StatusCode::PAYMENT_REQUIRED,
            format!("PAYMENT-SIGNATURE did not contain JSON: {error}"),
        )
    })
}

async fn pin_art_and_metadata(
    config: &ShopConfig,
    token_id: u64,
    prompt: &str,
    svg: &str,
) -> Result<String, (StatusCode, String)> {
    let pinata_jwt = config.ipfs_pinata_jwt.as_deref().ok_or_else(|| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            "IPFS_PINATA_JWT is required for on-chain fulfillment".to_owned(),
        )
    })?;
    let client = reqwest::Client::new();
    let image_part = multipart::Part::text(svg.to_owned())
        .file_name(format!("0xfarmer-{token_id}.svg"))
        .mime_str("image/svg+xml")
        .map_err(|error| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to build IPFS image part: {error}"),
            )
        })?;
    let image_form = multipart::Form::new().part("file", image_part);
    let image_cid = pinata_post_multipart(
        &client,
        pinata_jwt,
        "https://api.pinata.cloud/pinning/pinFileToIPFS",
        image_form,
    )
    .await?;
    let image_uri = format!("ipfs://{image_cid}");

    let metadata = NftMetadata {
        name: format!("0xFarmer Agent Shop #{token_id:03}"),
        description: "A paid x402 0xFarmer Agent Shop mint on Base mainnet.",
        image: image_uri,
        attributes: vec![
            NftAttribute {
                trait_type: "Network",
                value: "Base".to_owned(),
            },
            NftAttribute {
                trait_type: "Prompt",
                value: prompt.chars().take(80).collect(),
            },
        ],
    };
    let metadata_cid = pinata_post_json(&client, pinata_jwt, &metadata).await?;
    Ok(format!("ipfs://{metadata_cid}"))
}

async fn pinata_post_multipart(
    client: &reqwest::Client,
    jwt: &str,
    url: &str,
    form: multipart::Form,
) -> Result<String, (StatusCode, String)> {
    let response = client
        .post(url)
        .bearer_auth(jwt)
        .multipart(form)
        .send()
        .await
        .map_err(|error| {
            (
                StatusCode::BAD_GATEWAY,
                format!("IPFS image pinning request failed: {error}"),
            )
        })?;
    parse_pinata_response(response).await
}

async fn pinata_post_json<T: Serialize>(
    client: &reqwest::Client,
    jwt: &str,
    body: &T,
) -> Result<String, (StatusCode, String)> {
    let response = client
        .post("https://api.pinata.cloud/pinning/pinJSONToIPFS")
        .bearer_auth(jwt)
        .json(body)
        .send()
        .await
        .map_err(|error| {
            (
                StatusCode::BAD_GATEWAY,
                format!("IPFS metadata pinning request failed: {error}"),
            )
        })?;
    parse_pinata_response(response).await
}

async fn parse_pinata_response(
    response: reqwest::Response,
) -> Result<String, (StatusCode, String)> {
    if !response.status().is_success() {
        return Err((
            StatusCode::BAD_GATEWAY,
            "IPFS pinning provider rejected the pin request".to_owned(),
        ));
    }
    let value = response
        .json::<serde_json::Value>()
        .await
        .map_err(|error| {
            (
                StatusCode::BAD_GATEWAY,
                format!("IPFS pinning response was not JSON: {error}"),
            )
        })?;
    if let Ok(parsed) = serde_json::from_value::<PinataResponsePascal>(value.clone()) {
        return Ok(parsed.ipfs_hash);
    }
    if let Ok(parsed) = serde_json::from_value::<PinataResponseCamel>(value) {
        if let Some(cid) = parsed.ipfs_hash.or(parsed.cid) {
            return Ok(cid);
        }
    }
    Err((
        StatusCode::BAD_GATEWAY,
        "IPFS pinning response did not include a CID".to_owned(),
    ))
}

async fn mint_on_base(
    config: &ShopConfig,
    wallet: &str,
    token_uri: &str,
) -> Result<ChainMintResult, (StatusCode, String)> {
    let _contract_address = config.nft_contract_address.as_deref().ok_or_else(|| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            "SHOP_NFT_CONTRACT_ADDRESS is required for on-chain fulfillment".to_owned(),
        )
    })?;
    let mut parts = config.mint_command.split_whitespace();
    let program = parts.next().ok_or_else(|| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            "SHOP_MINT_COMMAND is empty".to_owned(),
        )
    })?;
    let mut command = Command::new(program);
    command.args(parts);
    command.arg(wallet).arg(token_uri);
    command.env(
        "SHOP_NFT_CONTRACT_ADDRESS",
        config.nft_contract_address.as_deref().unwrap_or_default(),
    );
    command.env("PUBLIC_BASE_URL", &config.public_base_url);
    command.env("SHOP_TREASURY_ADDRESS", &config.treasury_address);

    let output = command.output().await.map_err(|error| {
        (
            StatusCode::BAD_GATEWAY,
            format!("Base mint command failed to start: {error}"),
        )
    })?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err((
            StatusCode::BAD_GATEWAY,
            format!("Base mint command failed: {stderr}"),
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value = serde_json::from_str(&stdout).map_err(|error| {
        (
            StatusCode::BAD_GATEWAY,
            format!("Base mint command did not return JSON: {error}"),
        )
    })?;
    serde_json::from_value::<ChainMintResult>(value).map_err(|error| {
        (
            StatusCode::BAD_GATEWAY,
            format!(
                "Base mint command JSON missing tokenId/totalSupply/lastMintedAt/txHash: {error}"
            ),
        )
    })
}

fn validate_wallet(wallet: &str) -> Result<String, (StatusCode, String)> {
    let wallet = wallet.trim().to_lowercase();
    if !is_likely_wallet(&wallet) {
        return Err((
            StatusCode::BAD_REQUEST,
            "wallet must look like a 0x EVM address".to_owned(),
        ));
    }
    Ok(wallet)
}

fn is_likely_wallet(wallet: &str) -> bool {
    wallet.len() == 42
        && wallet.starts_with("0x")
        && wallet.chars().skip(2).all(|c| c.is_ascii_hexdigit())
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs()
}

fn escape_xml(value: &str) -> String {
    value
        .chars()
        .take(80)
        .flat_map(|c| match c {
            '&' => "&amp;".chars().collect::<Vec<_>>(),
            '<' => "&lt;".chars().collect(),
            '>' => "&gt;".chars().collect(),
            '"' => "&quot;".chars().collect(),
            '\'' => "&apos;".chars().collect(),
            _ => vec![c],
        })
        .collect()
}

fn generate_pixel_shop_svg(token_id: u64, prompt: &str) -> String {
    let prompt = escape_xml(prompt);
    let hue = (token_id * 37) % 360;
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" shape-rendering="crispEdges">
  <rect width="512" height="512" fill="#14333a"/>
  <rect x="32" y="352" width="448" height="96" fill="#1c1712"/>
  <rect x="72" y="184" width="368" height="184" fill="#d9a857"/>
  <rect x="56" y="144" width="400" height="56" fill="#7a1f24"/>
  <rect x="88" y="128" width="336" height="32" fill="#f2d27a"/>
  <rect x="104" y="208" width="104" height="144" fill="#4a2618"/>
  <rect x="248" y="220" width="144" height="92" fill="#101820"/>
  <rect x="264" y="236" width="112" height="60" fill="hsl({hue} 72% 62%)"/>
  <rect x="116" y="232" width="28" height="28" fill="#ffd86f"/>
  <rect x="168" y="232" width="28" height="28" fill="#ffd86f"/>
  <rect x="110" y="96" width="36" height="48" fill="#ffefd0"/>
  <rect x="366" y="96" width="36" height="48" fill="#ffefd0"/>
  <text x="256" y="181" text-anchor="middle" font-family="monospace" font-size="28" font-weight="700" fill="#fff7d7">0xFARMER 屋</text>
  <text x="256" y="407" text-anchor="middle" font-family="monospace" font-size="20" fill="#ffd86f">TOKEN #{token_id:03}</text>
  <text x="256" y="434" text-anchor="middle" font-family="monospace" font-size="14" fill="#fff7d7">{prompt}</text>
  <rect x="0" y="0" width="512" height="512" fill="none" stroke="#ffd86f" stroke-width="16"/>
</svg>"##
    )
}

fn local_metadata(token_id: u64, prompt: &str, svg: &str) -> NftMetadata<'static> {
    NftMetadata {
        name: format!("0xFarmer Agent Shop #{token_id:03}"),
        description: "A paid AgentCash/x402 0xFarmer Agent Shop mint. Local fulfillment returns embedded SVG metadata; production on-chain fulfillment pins this metadata to IPFS before minting.",
        image: format!("data:image/svg+xml;base64,{}", STANDARD.encode(svg)),
        attributes: vec![
            NftAttribute {
                trait_type: "Network",
                value: "Base".to_owned(),
            },
            NftAttribute {
                trait_type: "Prompt",
                value: prompt.chars().take(80).collect(),
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wallet(n: u8) -> String {
        format!("0x{n:040x}")
    }

    #[test]
    fn payment_requirements_are_x402_v2_base_mainnet_agentcash_treasury_and_usdc() {
        let reqs = payment_requirements();
        let accept = &reqs.accepts[0];

        assert_eq!(reqs.x402_version, 2);
        assert_eq!(reqs.resource.url, "http://127.0.0.1:3000/api/shop/mint");
        assert_eq!(accept.network, "eip155:8453");
        assert_eq!(accept.amount, "1000000");
        assert_eq!(accept.asset, "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913");
        assert_eq!(accept.pay_to, "0x73731620Bd6bB9EDc3c77E5982665fCBA797D710");
        assert!(!reqs.error.contains("Local dev accepts"));
    }

    #[test]
    fn endpoint_cooldown_blocks_different_wallet_after_success() {
        let state = ShopState::default();

        state
            .record_successful_mint_for_test(
                wallet(1),
                "first".to_owned(),
                "ipfs://metadata-one".to_owned(),
                100,
            )
            .expect("first mint succeeds");
        let second = state.record_successful_mint_for_test(
            wallet(2),
            "second".to_owned(),
            "ipfs://metadata-two".to_owned(),
            100,
        );

        assert_eq!(second.unwrap_err().0, StatusCode::TOO_MANY_REQUESTS);
    }

    #[test]
    fn mint_record_and_response_use_ipfs_metadata_uri_not_data_url() {
        let state = ShopState::default();
        let minted = state
            .record_successful_mint_for_test(
                wallet(3),
                "prompt".to_owned(),
                "ipfs://bafy/metadata.json".to_owned(),
                1000,
            )
            .expect("mint succeeds");

        assert_eq!(minted.token_uri, "ipfs://bafy/metadata.json");
        assert!(
            !serde_json::to_string(&minted)
                .unwrap()
                .contains("data:image")
        );
        assert_eq!(
            state.status().latest[0].token_uri,
            "ipfs://bafy/metadata.json"
        );
    }

    #[test]
    fn shop_config_defaults_to_agentcash_facilitator_and_local_fulfillment_for_paid_mints() {
        unsafe {
            std::env::remove_var("X402_FACILITATOR_URL");
            std::env::remove_var("X402_SETTLE_URL");
            std::env::remove_var("SHOP_FULFILLMENT_MODE");
            std::env::remove_var("IPFS_PINATA_JWT");
            std::env::remove_var("SHOP_NFT_CONTRACT_ADDRESS");
        }

        let config =
            ShopConfig::from_env_for_paid_mint().expect("default local AgentCash config works");

        assert_eq!(config.x402_facilitator_url, "https://x402.org/facilitator");
        assert_eq!(config.fulfillment_mode, FulfillmentMode::Local);
    }

    #[test]
    fn shop_config_accepts_settle_url_and_normalizes_facilitator_base_url() {
        unsafe {
            std::env::remove_var("X402_FACILITATOR_URL");
            std::env::set_var("X402_SETTLE_URL", "https://x402.org/facilitator/settle");
            std::env::set_var("SHOP_FULFILLMENT_MODE", "local");
        }

        let config = ShopConfig::from_env_for_paid_mint().expect("settle url config works");

        assert_eq!(config.x402_facilitator_url, "https://x402.org/facilitator");
        assert_eq!(config.fulfillment_mode, FulfillmentMode::Local);

        unsafe {
            std::env::remove_var("X402_SETTLE_URL");
            std::env::remove_var("SHOP_FULFILLMENT_MODE");
        }
    }

    #[test]
    fn verify_only_or_failed_settlement_response_is_rejected() {
        let response = X402SettleResponse {
            success: false,
            transaction: None,
            error_reason: Some("verified-but-not-settled".to_owned()),
            error_message: Some("verify-only response cannot mint".to_owned()),
        };

        let error = require_successful_settlement(response).unwrap_err();

        assert_eq!(error.0, StatusCode::PAYMENT_REQUIRED);
        assert!(error.1.contains("verified-but-not-settled"));
    }

    #[test]
    fn payment_response_header_encodes_settlement_response() {
        let response = X402SettleResponse {
            success: true,
            transaction: Some("0xsettled".to_owned()),
            error_reason: None,
            error_message: None,
        };

        let header = encode_payment_response_header(&response);
        let decoded: serde_json::Value =
            serde_json::from_slice(&STANDARD.decode(header).unwrap()).unwrap();

        assert_eq!(decoded["success"], true);
        assert_eq!(decoded["transaction"], "0xsettled");
    }

    #[test]
    fn durable_state_reloads_minted_supply_latest_and_consumed_payments() {
        let path = std::env::temp_dir().join(format!("shop-state-{}.json", unix_now()));
        let state = ShopState::with_state_path(&path);
        state
            .reserve_payment_header("paid-payload")
            .expect("reserve payment");
        state
            .record_successful_mint_for_test(
                wallet(4),
                "durable".to_owned(),
                "ipfs://durable-metadata".to_owned(),
                10_000,
            )
            .expect("mint succeeds");

        let restarted = ShopState::with_state_path(&path);
        assert_eq!(restarted.status().minted, 1);
        assert_eq!(restarted.status().latest[0].token_id, 1);
        assert_eq!(
            restarted
                .reserve_payment_header("paid-payload")
                .unwrap_err()
                .0,
            StatusCode::CONFLICT
        );
        let _ = fs::remove_file(path);
    }
}
