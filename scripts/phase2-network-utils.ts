/**
 * Phase 2 Example: Network Utilities
 *
 * Demonstrates:
 * - Checking account balances
 * - Requesting tokens from faucet
 * - Gas estimation
 * - Network information
 */

import {
  NetworkHelper,
  Network,
  Signer,
  getContract,
  balance,
  decodeBalance,
  formatBalance,
} from '../packages/sdk/src';
import { get_dev_account, get_address } from 'glin_client';

async function main() {
  console.log('💰 Phase 2: Network Utilities Demo\n');

  const helper = new NetworkHelper(Network.Testnet);

  // Get Alice's address
  const aliceKeypair = get_dev_account('alice');
  const aliceAddress = get_address(aliceKeypair);

  console.log('📊 Example 1: Check Balance');
  console.log(`Account: ${aliceAddress}\n`);

  const balanceInfo = await helper.getBalance(aliceAddress);
  console.log('  Raw balance:', balanceInfo.raw);
  console.log('  Decimal value:', balanceInfo.value);
  console.log('  Formatted:', balanceInfo.formatted);

  console.log('\n💵 Example 2: Request Faucet (Testnet Only)');
  console.log('Requesting 100 GLIN...\n');

  try {
    const faucetResult = await helper.requestFaucet(aliceAddress);
    if (faucetResult.success) {
      console.log('  ✓ Received:', faucetResult.amount);
      console.log('  Transaction:', faucetResult.txHash);
    }
  } catch (error: any) {
    console.log('  ℹ️ Faucet may not be available:', error.message);
  }

  console.log('\n⚡ Example 3: Gas Estimation');

  // Replace with actual contract address
  const contractAddress = '5GrwvaEF5YjHFPyNt1qrQKxDHUVwvSLfGBj6TT6u7o6Y8jKt';

  try {
    const gasEstimate = await helper.estimateGas({
      address: contractAddress,
      method: 'transfer',
      args: [aliceAddress, '1000'],
      from: 'alice',
    });

    console.log('  Gas limit:', gasEstimate.gasLimit);
    console.log('  Estimated cost:', gasEstimate.estimatedCost);
  } catch (error: any) {
    console.log('  ℹ️ Gas estimation:', error.message);
  }

  console.log('\n🌐 Example 4: Network Information');

  const networkInfo = await helper.getNetworkInfo();
  console.log('  Network:', networkInfo.name);
  console.log('  RPC:', networkInfo.rpc);
  console.log('  Current block:', networkInfo.blockNumber);

  console.log('\n🔢 Example 5: Balance Helpers');

  // Encode balance: 1 GLIN with 18 decimals
  const oneGlin = balance(1, 18);
  console.log('  1 GLIN encoded:', oneGlin);

  // Decode balance
  const decoded = decodeBalance(oneGlin, 18);
  console.log('  Decoded back:', decoded);

  // Format for display
  const formatted = formatBalance(oneGlin, 18, 4, 'GLIN');
  console.log('  Formatted:', formatted);

  // Use with BigInt
  const bigAmount = 1_500_000_000_000_000_000n; // 1.5 GLIN
  const formattedBig = formatBalance(bigAmount, 18, 2, 'GLIN');
  console.log('  1.5 GLIN formatted:', formattedBig);
}

main()
  .then(() => {
    console.log('\n✨ Network utilities demo complete!');
    process.exit(0);
  })
  .catch((error) => {
    console.error('\n❌ Error:', error.message);
    process.exit(1);
  });
