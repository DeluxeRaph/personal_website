import { readFileSync, existsSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { ContractFactory, JsonRpcProvider, Wallet } from 'ethers';

const rpcUrl = process.env.BASE_RPC_URL || process.env.RPC_URL;
const privateKey = process.env.DEPLOYER_PRIVATE_KEY || process.env.PRIVATE_KEY;
const minter = process.env.SHOP_MINTER_ADDRESS;
const expectedChainId = 8453n;

if (!rpcUrl || !privateKey) {
  console.error('Set BASE_RPC_URL/RPC_URL and DEPLOYER_PRIVATE_KEY/PRIVATE_KEY before deploying to Base mainnet.');
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
  console.error(`Refusing to deploy: RPC chainId ${network.chainId} is not Base mainnet (${expectedChainId}).`);
  process.exit(1);
}

const wallet = new Wallet(privateKey, provider);
const minterAddress = minter || wallet.address;

console.log(`Deploying FarmerShopNFT to Base mainnet from ${wallet.address}`);
console.log(`Initial minter: ${minterAddress}`);
const factory = new ContractFactory(artifact.abi, artifact.bytecode, wallet);
const contract = await factory.deploy(minterAddress);
await contract.waitForDeployment();
const address = await contract.getAddress();

const launch = {
  network: 'base',
  chainId: Number(network.chainId),
  contractAddress: address,
  minterAddress,
  deployedAt: new Date().toISOString()
};
writeFileSync('farmer-shop.launch.json', `${JSON.stringify(launch, null, 2)}\n`);
console.log(JSON.stringify(launch, null, 2));
