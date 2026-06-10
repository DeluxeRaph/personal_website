# Raphael Nembhard Personal Site

A Rust + Axum personal site with a Windows 95-inspired interface.

## Run

```bash
cargo run
```

Then open <http://127.0.0.1:3000>.

## Production-style Run

Local defaults are `HOST=127.0.0.1` and `PORT=3000`.

```bash
HOST=0.0.0.0 PORT=3000 cargo run --release
```

Or keep it alive with PM2:

```bash
npm install
npx pm2 start ecosystem.config.cjs
npx pm2 save
```

Health check:

```bash
curl http://127.0.0.1:3000/healthz
```

## 0xFarmer Agent Shop

The shop lives at `/shop` and exposes launch-oriented Base mainnet API endpoints:

- `GET /api/shop/status` returns price, supply, endpoint cooldown, Base chain metadata, AgentCash treasury, and latest token metadata URIs.
- `POST /api/shop/mint` without `PAYMENT-SIGNATURE` returns a `402 Payment Required` JSON body plus a `PAYMENT-REQUIRED` header for Base mainnet USDC (`0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913`) and payTo `0x73731620Bd6bB9EDc3c77E5982665fCBA797D710` unless `SHOP_TREASURY_ADDRESS` overrides it.
- `POST /api/shop/mint` with `PAYMENT-SIGNATURE` now fails closed unless launch integrations are configured. The server settles the x402 v2 payment through `X402_FACILITATOR_URL`/`X402_SETTLE_URL`, rejects already-consumed payment payloads, pins SVG art plus JSON metadata to IPFS through Pinata, then calls the Base mainnet NFT contract minter. Verify-only payment responses are not sufficient to mint.

```bash
npx agentcash@latest fetch http://127.0.0.1:3000/api/shop/mint \
  -m POST \
  -b '{"wallet":"0xYourWallet","prompt":"8 bit farmer at a Japanese lantern shop"}'
```

- After AgentCash retries with `PAYMENT-SIGNATURE`, the server calls the x402 facilitator `/settle` endpoint before doing any mint work. In explicit `SHOP_FULFILLMENT_MODE=local`, it returns paid embedded metadata for testing. In default/production `SHOP_FULFILLMENT_MODE=onchain`, it pins SVG art plus JSON metadata to IPFS through Pinata, then calls the Base mainnet NFT contract minter and records the on-chain token ID/total supply.

Required launch environment:

```bash
PUBLIC_BASE_URL=https://0xfarmer.com
X402_FACILITATOR_URL=https://x402.org/facilitator
SHOP_FULFILLMENT_MODE=onchain
IPFS_PINATA_JWT=<pinata jwt>
BASE_RPC_URL=<base mainnet rpc>
SHOP_NFT_CONTRACT_ADDRESS=<deployed Base mainnet contract>
SHOP_MINTER_PRIVATE_KEY=<minter key allowed by the contract>
# optional: SHOP_TREASURY_ADDRESS defaults to AgentCash Base address above
# optional: SHOP_MINT_COMMAND="node scripts/mint-farmer-token.mjs"
```

Cooldown is endpoint/global-level: after one successful paid mint, any wallet must wait 5 minutes before the next paid mint. Payment verification, IPFS pinning, and the on-chain mint must all succeed before the cooldown/supply counter is advanced.

## Smart Contract

A minimal NFT minter is in `contracts/FarmerShopNFT.sol`. Payments happen off-chain through x402; the contract enforces max supply, endpoint-level cooldown, `ipfs://` token URIs, and minter-only minting.

```bash
npm install
npm run contract:compile
BASE_RPC_URL=<base-mainnet-rpc> DEPLOYER_PRIVATE_KEY=<key> npm run contract:deploy:base
```

The deploy script refuses non-8453 RPCs and writes `farmer-shop.launch.json` with the deployed contract address. Runtime minting uses:

```bash
BASE_RPC_URL=<base-mainnet-rpc> \
SHOP_NFT_CONTRACT_ADDRESS=<contract> \
SHOP_MINTER_PRIVATE_KEY=<minter-key> \
npm run contract:mint:base -- <recipient-wallet> <ipfs://metadata-uri>
```

## Domain Notes

`0xfarmer.com` and `www.0xfarmer.com` did not resolve from this VPS during launch review. Registrar/DNS ownership is not available in this repository/session, so buying or setting the domain is blocked until registrar/DNS credentials are provided. Once available, point `A`/`AAAA` (or CNAME for `www`) to the production host, set `PUBLIC_BASE_URL=https://0xfarmer.com`, and terminate TLS with nginx/Caddy or another reverse proxy on ports 80/443.

## AI Runtime Notes

For the VPS image runtime, practical options are:

- Use Hermes/AgentCash `stablestudio.dev` for paid image generation while keeping this app as the x402 gate and NFT minter.
- Run ComfyUI locally if the VPS has a suitable GPU and enough VRAM; expose a small internal HTTP endpoint and call it after payment.
- Keep the built-in SVG generator as a cheap fallback so mints still work when the AI runtime is unavailable.

## Edit Content

Homepage rendering lives in `src/views.rs`, routes live in `src/app.rs` and `src/handlers.rs`, and static assets live in `assets/`.
