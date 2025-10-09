import { deploy, getConfig, initApi } from '@glin-forge/sdk';

async function main() {
  console.log('🚀 Deploying Token Contract...\n');

  // Initialize API
  const api = await initApi();
  const config = getConfig();

  // Get initial supply from config
  const initialSupply = config.vars?.initialSupply || '1000000000000000000000000';

  console.log('Configuration:');
  console.log(`  Network: ${config.defaultNetwork}`);
  console.log(`  Initial Supply: ${initialSupply}`);
  console.log('');

  // Deploy contract
  const result = await deploy('token', {
    constructorArgs: [initialSupply],
    gasLimit: 100000000000,
  });

  console.log('✓ Contract deployed successfully!\n');
  console.log('Deployment details:');
  console.log(`  Contract Address: ${result.address}`);
  console.log(`  Code Hash: ${result.codeHash}`);
  console.log(`  Transaction Hash: ${result.txHash}`);
  console.log('');

  // Query initial state
  const contract = result.contract;

  const totalSupply = await contract.query.totalSupply();
  const ownerBalance = await contract.query.balanceOf(result.deployer);

  console.log('Initial state:');
  console.log(`  Total Supply: ${totalSupply.output}`);
  console.log(`  Owner Balance: ${ownerBalance.output}`);
  console.log('');

  console.log('💡 Next steps:');
  console.log('  1. Update frontend config with contract address');
  console.log(`     → export const CONTRACT_ADDRESS = '${result.address}';`);
  console.log('  2. Start the frontend:');
  console.log('     → cd frontend && npm run dev');
  console.log('  3. Interact with the contract through the UI');
  console.log('');

  await api.disconnect();
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
