import { readFileSync, existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { Contract, JsonRpcProvider, Wallet, isAddress } from 'ethers';

const [to, tokenUri] = process.argv.slice(2);
const rpcUrl = process.env.BASE_RPC_URL || process.env.RPC_URL;
const privateKey = process.env.SHOP_MINTER_PRIVATE_KEY || process.env.DEPLOYER_PRIVATE_KEY || process.env.PRIVATE_KEY;
const contractAddress = process.env.SHOP_NFT_CONTRACT_ADDRESS;
const expectedChainId = 8453n;

if (!to || !tokenUri) {
  console.error('Usage: node scripts/mint-farmer-token.mjs <recipient> <ipfs://metadata-uri>');
  process.exit(1);
}
if (!isAddress(to)) {
  console.error('Recipient must be a valid EVM address.');
  process.exit(1);
}
if (!tokenUri.startsWith('ipfs://')) {
  console.error('Token URI must be an ipfs:// metadata URI.');
  process.exit(1);
}
if (!rpcUrl || !privateKey || !contractAddress) {
  console.error('Set BASE_RPC_URL/RPC_URL, SHOP_MINTER_PRIVATE_KEY/DEPLOYER_PRIVATE_KEY/PRIVATE_KEY, and SHOP_NFT_CONTRACT_ADDRESS.');
  process.exit(1);
}

const artifactPath = resolve('artifacts/FarmerShopNFT.json');
if (!existsSync(artifactPath)) {
  console.error('Missing artifact. Run: npm run contract:compile');
  process.exit(1);
}

const artifact = JSON.parse(readFileSync(artifactPath, 'utf8'));
const provider = new JsonRpcProvider(rpcUrl);
const network = await provider.getNetwork();
if (network.chainId !== expectedChainId) {
  console.error(`Refusing to mint: RPC chainId ${network.chainId} is not Base mainnet (${expectedChainId}).`);
  process.exit(1);
}

const signer = new Wallet(privateKey, provider);
const contract = new Contract(contractAddress, artifact.abi, signer);
const tx = await contract.mintTo(to, tokenUri);
const receipt = await tx.wait();
const transfer = receipt.logs
  .map((log) => {
    try {
      return contract.interface.parseLog(log);
    } catch {
      return null;
    }
  })
  .find((event) => event?.name === 'Transfer' && event.args?.from === '0x0000000000000000000000000000000000000000');

if (!transfer) {
  console.error('Mint receipt did not include a Transfer event from the zero address.');
  process.exit(1);
}

const tokenId = transfer.args.tokenId;
const totalSupply = await contract.totalSupply();
const lastMintedAt = await contract.lastMintedAt();
console.log(JSON.stringify({
  txHash: receipt.hash,
  tokenId: Number(tokenId),
  totalSupply: Number(totalSupply),
  lastMintedAt: Number(lastMintedAt),
  contractAddress,
  network: 'base',
  chainId: Number(network.chainId),
}));
