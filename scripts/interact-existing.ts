/**
 * Example: Interact with an existing deployed contract
 *
 * Run with: glin-forge run scripts/interact-existing.ts
 */

import { getContract, Network, Signer } from '../packages/sdk/src';

async function main() {
  // Replace with your deployed contract address
  const CONTRACT_ADDRESS = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

  console.log('Connecting to contract:', CONTRACT_ADDRESS);

  const contract = getContract({
    address: CONTRACT_ADDRESS,
    metadata: './target/ink/metadata.json',
    network: Network.Testnet,
    account: Signer.Alice,
  });

  // Query current state
  console.log('\nQuerying contract state...');
  const balance = await contract.query('balanceOf', [CONTRACT_ADDRESS]);
  console.log('Balance:', balance);

  // Call a method
  console.log('\nCalling transfer...');
  await contract.call('transfer', [
    '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
    '1000',
  ]);

  console.log('✅ Transaction sent!');
}

main().catch(console.error);
