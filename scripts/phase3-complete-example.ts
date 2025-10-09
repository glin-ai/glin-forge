/**
 * Phase 3 Complete Example: End-to-End Type-Safe Development
 *
 * This example shows a complete workflow from contract deployment
 * to type-safe interactions using generated types.
 *
 * Prerequisites:
 * 1. Contract compiled: cargo contract build
 * 2. Types generated: glin-forge typegen
 * 3. glin-forge run in another terminal
 */

import {
  deploy,
  getContract,
  Network,
  Signer,
  balance,
  formatBalance,
} from '../packages/sdk/src';

async function main() {
  console.log('🎯 Phase 3: Complete Type-Safe Workflow\n');
  console.log('='.repeat(70));

  // ========================================
  // Part 1: Deploy ERC20 Token
  // ========================================

  console.log('\n📦 Part 1: Deploy ERC20 Token Contract');
  console.log('-'.repeat(70));

  try {
    console.log('  Deploying token with:');
    console.log('    • Initial supply: 1,000,000 tokens');
    console.log('    • Name: "MyToken"');
    console.log('    • Symbol: "MTK"');
    console.log('    • Decimals: 18');

    const token = await deploy({
      wasm: './target/ink/erc20.wasm',
      metadata: './target/ink/erc20_metadata.json',
      args: [
        balance(1000000, 18), // 1 million tokens
        'MyToken',
        'MTK',
        18,
      ],
      network: Network.Testnet,
      account: Signer.Alice,
    });

    console.log('  ✓ Token deployed!');
    console.log('    Address:', token.address);

    // ========================================
    // Part 2: Type-Safe Queries
    // ========================================

    console.log('\n🔍 Part 2: Type-Safe Query Operations');
    console.log('-'.repeat(70));

    // Query token name (TypeScript knows this returns string)
    const name = await token.query('tokenName');
    console.log('  Token name:', name);

    // Query token symbol (TypeScript knows this returns string)
    const symbol = await token.query('tokenSymbol');
    console.log('  Token symbol:', symbol);

    // Query total supply (TypeScript knows this returns bigint)
    const totalSupply = await token.query('totalSupply');
    console.log('  Total supply:', formatBalance(totalSupply, 18, 2, 'MTK'));

    // Query Alice's balance
    const aliceAddr = '5GrwvaEF5YjHFPyNt1qrQKxDHUVwvSLfGBj6TT6u7o6Y8jKt';
    const aliceBalance = await token.query('balanceOf', [aliceAddr]);
    console.log('  Alice balance:', formatBalance(aliceBalance, 18, 2, 'MTK'));

    // ========================================
    // Part 3: Type-Safe Transactions
    // ========================================

    console.log('\n💸 Part 3: Type-Safe Transaction Operations');
    console.log('-'.repeat(70));

    const bobAddr = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';
    const transferAmount = balance(100, 18); // 100 MTK

    console.log('  Transferring 100 MTK to Bob...');
    console.log('    From:', aliceAddr);
    console.log('    To:', bobAddr);

    // TypeScript ensures we pass correct types
    const tx = await token.call('transfer', [bobAddr, transferAmount]);

    console.log('  ✓ Transaction submitted');
    console.log('    Hash:', tx.hash());

    console.log('  Waiting for finalization...');
    const receipt = await tx.wait();

    console.log('  ✓ Transaction finalized!');
    console.log('    Block number:', receipt.blockNumber);
    console.log('    Block hash:', receipt.blockHash);

    // ========================================
    // Part 4: Type-Safe Events
    // ========================================

    console.log('\n📡 Part 4: Type-Safe Event Watching');
    console.log('-'.repeat(70));

    // Create event watcher with type-safe event names
    const watcher = token.watch({ event: 'Transfer', limit: 5 });

    let eventCount = 0;

    // TypeScript knows the event data structure
    watcher.on('Transfer', (event) => {
      eventCount++;
      console.log(`  Event ${eventCount}:`);
      console.log('    From:', event.data.from);
      console.log('    To:', event.data.to);
      console.log('    Amount:', formatBalance(event.data.value, 18, 2, 'MTK'));
      console.log('    Block:', event.blockNumber);
    });

    console.log('  Fetching recent Transfer events...\n');
    await watcher.start();

    console.log(`\n  ✓ Found ${eventCount} Transfer events`);

    // ========================================
    // Part 5: Approval and Allowance
    // ========================================

    console.log('\n🔐 Part 5: Type-Safe Approval Flow');
    console.log('-'.repeat(70));

    const spender = bobAddr;
    const approvalAmount = balance(50, 18); // 50 MTK

    console.log(`  Approving Bob to spend 50 MTK...`);

    // TypeScript ensures correct parameter types
    const approveTx = await token.call('approve', [spender, approvalAmount]);
    await approveTx.wait();

    console.log('  ✓ Approval confirmed');

    // Query allowance (TypeScript knows this returns bigint)
    const allowance = await token.query('allowance', [aliceAddr, spender]);
    console.log('  Allowance:', formatBalance(allowance, 18, 2, 'MTK'));

    // ========================================
    // Part 6: Type Safety in Action
    // ========================================

    console.log('\n✨ Part 6: Type Safety Examples');
    console.log('-'.repeat(70));

    console.log(`
  The following code would NOT compile with TypeScript:

  ❌ Wrong method name:
     await token.query('getBalance', [addr]);
     // Error: 'getBalance' does not exist on queries

  ❌ Wrong parameter type:
     await token.call('transfer', ['Bob', '100']);
     // Error: Expected [string, bigint], got [string, string]

  ❌ Wrong number of parameters:
     await token.query('balanceOf');
     // Error: Expected 1 argument, got 0

  ✅ Correct usage:
     await token.query('balanceOf', [address]);
     // TypeScript is happy! ✓
    `);

    // ========================================
    // Summary
    // ========================================

    console.log('\n' + '='.repeat(70));
    console.log('✅ Complete Type-Safe Workflow Demonstrated!');
    console.log('='.repeat(70));

    console.log('\nOperations Performed:');
    console.log('  ✓ Type-safe contract deployment');
    console.log('  ✓ Type-safe queries (name, symbol, balance, etc.)');
    console.log('  ✓ Type-safe transactions (transfer, approve)');
    console.log('  ✓ Type-safe event watching');
    console.log('  ✓ Compile-time error prevention');

    console.log('\nBenefits of Type Safety:');
    console.log('  • Catch errors before runtime');
    console.log('  • IDE autocomplete for all methods');
    console.log('  • Self-documenting code');
    console.log('  • Safe refactoring');
    console.log('  • Better developer experience');

  } catch (error: any) {
    console.error('\n❌ Error:', error.message);
    console.log('\nℹ️  This is a demonstration script.');
    console.log('    Make sure you have:');
    console.log('    1. Compiled contract: cargo contract build');
    console.log('    2. Generated types: glin-forge typegen');
    console.log('    3. Running RPC: glin-forge run');
  }
}

main()
  .then(() => {
    console.log('\n✨ Complete example finished!');
    process.exit(0);
  })
  .catch((error) => {
    console.error('\n❌ Fatal error:', error.message);
    process.exit(1);
  });
