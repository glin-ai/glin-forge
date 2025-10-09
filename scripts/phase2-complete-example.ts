/**
 * Phase 2 Complete Example
 *
 * A comprehensive demo showing all Phase 2 features working together:
 * - Deploy contract
 * - Check balances
 * - Send transactions with wait()
 * - Watch events
 * - Use type marshaling
 */

import {
  deploy,
  getContract,
  Network,
  Signer,
  NetworkHelper,
  balance,
  formatBalance,
} from '../packages/sdk/src';

async function main() {
  console.log('🚀 Phase 2: Complete Feature Demo\n');
  console.log('='.repeat(60));

  const helper = new NetworkHelper(Network.Testnet);

  // Step 1: Check initial balance
  console.log('\n📊 Step 1: Check Alice\'s Balance');
  console.log('-'.repeat(60));

  try {
    const aliceAddress = '5GrwvaEF5YjHFPyNt1qrQKxDHUVwvSLfGBj6TT6u7o6Y8jKt';
    const balanceInfo = await helper.getBalance(aliceAddress);

    console.log('  Account:', aliceAddress);
    console.log('  Balance:', balanceInfo.formatted);
  } catch (error: any) {
    console.log('  ℹ️ ', error.message);
  }

  // Step 2: Deploy contract
  console.log('\n📦 Step 2: Deploy Contract');
  console.log('-'.repeat(60));

  try {
    console.log('  Deploying ERC20 token...');

    const contract = await deploy({
      wasm: './target/ink/erc20.wasm',
      metadata: './target/ink/metadata.json',
      args: [
        balance(1000000, 18), // Initial supply: 1 million tokens
        'MyToken', // Name
        'MTK', // Symbol
        18, // Decimals
      ],
      network: Network.Testnet,
      account: Signer.Alice,
    });

    console.log('  ✓ Contract deployed!');
    console.log('    Address:', contract.address);

    // Step 3: Query contract
    console.log('\n🔍 Step 3: Query Contract State');
    console.log('-'.repeat(60));

    const totalSupply = await contract.query('totalSupply');
    console.log('  Total supply:', formatBalance(totalSupply, 18, 2, 'MTK'));

    const name = await contract.query('tokenName');
    console.log('  Token name:', name);

    // Step 4: Send transaction with wait
    console.log('\n💸 Step 4: Transfer Tokens');
    console.log('-'.repeat(60));

    const recipient = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';
    const transferAmount = balance(100, 18);

    console.log('  Transferring 100 MTK to:', recipient);
    console.log('  Amount:', formatBalance(transferAmount, 18, 2, 'MTK'));

    const tx = await contract.call('transfer', [
      recipient,
      transferAmount,
    ]);

    console.log('  ✓ Transaction submitted:', tx.hash());
    console.log('  Waiting for finalization...');

    const receipt = await tx.wait();

    console.log('  ✓ Transaction finalized!');
    console.log('    Block:', receipt.blockNumber);
    console.log('    Hash:', receipt.blockHash);

    // Step 5: Verify balances
    console.log('\n🧮 Step 5: Verify Balances');
    console.log('-'.repeat(60));

    const senderBalance = await contract.query('balanceOf', [
      '5GrwvaEF5YjHFPyNt1qrQKxDHUVwvSLfGBj6TT6u7o6Y8jKt',
    ]);

    const recipientBalance = await contract.query('balanceOf', [recipient]);

    console.log('  Sender balance:', formatBalance(senderBalance, 18, 2, 'MTK'));
    console.log('  Recipient balance:', formatBalance(recipientBalance, 18, 2, 'MTK'));

    // Step 6: Watch events
    console.log('\n📡 Step 6: Watch Transfer Events');
    console.log('-'.repeat(60));

    const watcher = contract.watch({
      event: 'Transfer',
      limit: 5,
    });

    let eventCount = 0;

    watcher.on('Transfer', (event) => {
      eventCount++;
      console.log(`  Event ${eventCount} at block ${event.blockNumber}:`);
      console.log('    From:', event.data.from);
      console.log('    To:', event.data.to);
      console.log('    Amount:', formatBalance(event.data.value, 18, 2, 'MTK'));
    });

    console.log('  Fetching recent events...\n');
    await watcher.start();

    console.log(`\n  ✓ Found ${eventCount} Transfer events`);

    // Step 7: Network info
    console.log('\n🌐 Step 7: Network Information');
    console.log('-'.repeat(60));

    const networkInfo = await helper.getNetworkInfo();

    console.log('  Network:', networkInfo.name);
    console.log('  RPC:', networkInfo.rpc);
    console.log('  Current block:', networkInfo.blockNumber);

    // Success summary
    console.log('\n' + '='.repeat(60));
    console.log('✅ All Phase 2 Features Demonstrated Successfully!');
    console.log('='.repeat(60));

    console.log('\nPhase 2 Features Used:');
    console.log('  ✓ Event watching (historical)');
    console.log('  ✓ Type marshaling (ArgumentEncoder)');
    console.log('  ✓ Balance helpers (encode/decode/format)');
    console.log('  ✓ Network utilities (balance, info)');
    console.log('  ✓ Transaction class (wait for finalization)');
    console.log('  ✓ Contract interactions (deploy, call, query)');

  } catch (error: any) {
    console.error('\n❌ Error:', error.message);
    console.log('\nℹ️  This is a demonstration script.');
    console.log('    Ensure you have a running testnet and valid contract files.');
  }
}

main()
  .then(() => {
    console.log('\n✨ Complete demo finished!');
    process.exit(0);
  })
  .catch((error) => {
    console.error('\n❌ Fatal error:', error.message);
    process.exit(1);
  });
