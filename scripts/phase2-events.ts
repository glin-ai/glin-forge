/**
 * Phase 2 Example: Event Watching
 *
 * Demonstrates:
 * - Historical event watching
 * - Real-time event streaming (follow mode)
 * - Event filtering
 * - Multiple event listeners
 */

import { getContract, EventWatcher, Network, Signer } from '../packages/sdk/src';

async function main() {
  console.log('🔍 Phase 2: Event Watching Demo\n');

  // Replace with actual deployed contract address
  const contractAddress = '5GrwvaEF5YjHFPyNt1qrQKxDHUVwvSLfGBj6TT6u7o6Y8jKt';

  const contract = getContract({
    address: contractAddress,
    metadata: './target/ink/metadata.json',
    network: Network.Testnet,
    account: Signer.Alice,
  });

  console.log('📚 Example 1: Historical Events');
  console.log('Getting last 10 Transfer events...\n');

  const watcher = contract.watch({
    event: 'Transfer',
    limit: 10,
  });

  watcher.on('Transfer', (event) => {
    console.log('  Transfer event:', {
      block: event.blockNumber,
      data: event.data,
    });
  });

  await watcher.start();

  console.log('\n📡 Example 2: Real-time Events (Follow Mode)');
  console.log('Watching for new events (press Ctrl+C to stop)...\n');

  const liveWatcher = contract.watch({
    follow: true,
    limit: 5,
  });

  liveWatcher.on('*', (event) => {
    console.log(`  ${event.eventName} at block ${event.blockNumber}:`, event.data);
  });

  await liveWatcher.start();
}

main()
  .then(() => {
    console.log('\n✨ Event watching demo complete!');
    process.exit(0);
  })
  .catch((error) => {
    console.error('\n❌ Error:', error.message);
    process.exit(1);
  });
