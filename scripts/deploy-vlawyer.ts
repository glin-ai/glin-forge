/**
 * Example deployment script for v-Lawyer contracts
 *
 * Run with: glin-forge run scripts/deploy-vlawyer.ts
 */

import { deploy, Network, Signer } from '../packages/sdk/src';

async function main() {
  console.log('🚀 Deploying v-Lawyer contracts...\n');

  // Deploy 3 v-Lawyer contracts in parallel
  const contracts = await Promise.all([
    deploy({
      wasm: './target/ink/vlawyer.wasm',
      metadata: './target/ink/metadata.json',
      args: ['VLawyer-1'],
      network: Network.Testnet,
      account: Signer.Alice,
    }),
    deploy({
      wasm: './target/ink/vlawyer.wasm',
      metadata: './target/ink/metadata.json',
      args: ['VLawyer-2'],
      network: Network.Testnet,
      account: Signer.Alice,
    }),
    deploy({
      wasm: './target/ink/vlawyer.wasm',
      metadata: './target/ink/metadata.json',
      args: ['VLawyer-3'],
      network: Network.Testnet,
      account: Signer.Alice,
    }),
  ]);

  console.log('\n✅ Deployment complete!\n');
  console.log('Deployed contracts:');
  contracts.forEach((contract, i) => {
    console.log(`  VLawyer-${i + 1}: ${contract.address}`);
  });

  // Example: Query the first contract
  console.log('\n📋 Querying VLawyer-1...');
  try {
    const name = await contracts[0].query('getName', []);
    console.log(`  Name: ${name}`);
  } catch (error) {
    console.log('  Query failed (contract might not have getName method)');
  }
}

// Run main and handle errors
main()
  .then(() => {
    console.log('\n✨ Script completed successfully!');
    process.exit(0);
  })
  .catch((error) => {
    console.error('\n❌ Script failed:', error.message);
    process.exit(1);
  });
