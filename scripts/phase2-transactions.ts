/**
 * Phase 2 Example: Transaction Class & Type Marshaling
 *
 * Demonstrates:
 * - Transaction class with wait() method
 * - Type-safe argument encoding
 * - BigInt support
 * - Complex type marshaling (arrays, objects)
 */

import {
  getContract,
  Network,
  Signer,
  ArgumentEncoder,
  balance,
} from '../packages/sdk/src';

async function main() {
  console.log('💸 Phase 2: Transactions & Type Marshaling Demo\n');

  // Replace with actual deployed contract address
  const contractAddress = '5GrwvaEF5YjHFPyNt1qrQKxDHUVwvSLfGBj6TT6u7o6Y8jKt';
  const recipientAddress = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';

  const contract = getContract({
    address: contractAddress,
    metadata: './target/ink/metadata.json',
    network: Network.Testnet,
    account: Signer.Alice,
  });

  console.log('📝 Example 1: Type Marshaling with ArgumentEncoder');

  // Encode simple types
  console.log('\n  Simple types:');
  console.log('    String:', ArgumentEncoder.encode('hello'));
  console.log('    Number:', ArgumentEncoder.encode(42));
  console.log('    Boolean:', ArgumentEncoder.encode(true));
  console.log('    BigInt:', ArgumentEncoder.encode(1000000000000000000n));

  // Encode arrays
  console.log('\n  Arrays:');
  const arrayArgs = ArgumentEncoder.encode([1, 2, 3]);
  console.log('    Array:', arrayArgs);

  // Encode objects (structs)
  console.log('\n  Objects (Structs):');
  const structArgs = ArgumentEncoder.encode({
    name: 'Alice',
    age: 30,
    active: true,
  });
  console.log('    Struct:', structArgs);

  // Encode balance
  console.log('\n  Balance encoding:');
  const amount = balance(1.5, 18);
  console.log('    1.5 GLIN:', amount);

  console.log('\n💰 Example 2: Transaction with wait()');

  try {
    console.log('  Calling contract.transfer()...');

    const tx = await contract.call('transfer', [
      recipientAddress,
      1000, // 1000 smallest units
    ]);

    console.log('  ✓ Transaction submitted');
    console.log('    Hash:', tx.hash());
    console.log('    Success:', tx.isSuccess());

    if (tx.blockHash()) {
      console.log('    Block:', tx.blockHash());
    }

    console.log('\n  Waiting for finalization...');

    const receipt = await tx.wait();

    console.log('  ✓ Transaction finalized!');
    console.log('    Block number:', receipt.blockNumber);
    console.log('    Block hash:', receipt.blockHash);
  } catch (error: any) {
    console.log('  ℹ️ Transaction demo:', error.message);
  }

  console.log('\n🔄 Example 3: Transaction with progress callback');

  try {
    const tx2 = await contract.call('approve', [
      recipientAddress,
      5000,
    ]);

    console.log('  Transaction hash:', tx2.hash());
    console.log('  Waiting with progress updates...\n');

    const receipt = await tx2.waitWithProgress(
      (elapsed) => {
        console.log(`    ... ${(elapsed / 1000).toFixed(1)}s elapsed`);
      },
      60000
    );

    console.log('\n  ✓ Transaction finalized in block:', receipt.blockNumber);
  } catch (error: any) {
    console.log('  ℹ️ Transaction demo:', error.message);
  }

  console.log('\n🎯 Example 4: Complex Arguments');

  // Demonstrate encoding complex contract arguments
  const complexArgs = [
    recipientAddress, // AccountId
    balance(2.5, 18), // Balance with decimals
    true, // Boolean flag
    [1, 2, 3], // Array
    { // Struct
      id: 42,
      active: true,
    },
  ];

  console.log('  Complex args encoded:');
  const encoded = ArgumentEncoder.encodeAll(complexArgs);
  encoded.forEach((arg, i) => {
    console.log(`    Arg ${i}:`, arg);
  });

  console.log('\n📊 Example 5: Balance Math');

  // Show balance calculations
  const balance1 = balance(10, 18); // 10 GLIN
  const balance2 = balance(5.5, 18); // 5.5 GLIN

  console.log('  10 GLIN:', balance1);
  console.log('  5.5 GLIN:', balance2);

  // BigInt arithmetic
  const total = BigInt(balance1) + BigInt(balance2);
  console.log('  Total:', total.toString());

  // Decode and format
  const { decodeBalance, formatBalance } = await import('../packages/sdk/src');
  const totalDecoded = decodeBalance(total, 18);
  const totalFormatted = formatBalance(total, 18, 4, 'GLIN');

  console.log('  Total decoded:', totalDecoded);
  console.log('  Total formatted:', totalFormatted);
}

main()
  .then(() => {
    console.log('\n✨ Transactions & type marshaling demo complete!');
    process.exit(0);
  })
  .catch((error) => {
    console.error('\n❌ Error:', error.message);
    process.exit(1);
  });
