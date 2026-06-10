import { readFileSync, mkdirSync, writeFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import solc from 'solc';

const sourcePath = resolve('contracts/FarmerShopNFT.sol');
const source = readFileSync(sourcePath, 'utf8');

const input = {
  language: 'Solidity',
  sources: {
    'FarmerShopNFT.sol': { content: source }
  },
  settings: {
    optimizer: { enabled: true, runs: 200 },
    outputSelection: {
      '*': {
        '*': ['abi', 'evm.bytecode.object']
      }
    }
  }
};

const output = JSON.parse(solc.compile(JSON.stringify(input)));
const errors = output.errors || [];
for (const error of errors) {
  console.error(`${error.severity}: ${error.formattedMessage}`);
}
if (errors.some((error) => error.severity === 'error')) {
  process.exit(1);
}

const contract = output.contracts['FarmerShopNFT.sol'].FarmerShopNFT;
const artifactPath = resolve('artifacts/FarmerShopNFT.json');
mkdirSync(dirname(artifactPath), { recursive: true });
writeFileSync(artifactPath, JSON.stringify({
  abi: contract.abi,
  bytecode: `0x${contract.evm.bytecode.object}`
}, null, 2));
console.log(`Wrote ${artifactPath}`);
