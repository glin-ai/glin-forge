/**
 * Simple deployment example
 *
 * Run with: glin-forge run scripts/deploy-simple.ts
 */

import { deploy, Network, Signer } from '../packages/sdk/src';

async function main() {
  console.log('Deploying contract...');

  const contract = await deploy({
    wasm: './target/ink/my_contract.wasm',
    metadata: './target/ink/metadata.json',
    args: [],
    network: Network.Testnet,
    account: Signer.Alice,
  });

  console.log('✅ Contract deployed!');
  console.log('Address:', contract.address);

  // Example: Call a method
  console.log('\nCalling setMessage...');
  await contract.call('setMessage', ['Hello from glin-forge!'], 0);

  // Example: Query state
  console.log('\nQuerying getMessage...');
  const message = await contract.query('getMessage', []);
  console.log('Message:', message);
}

main().catch(console.error);
