/**
 * Phase 3 Example: Type-Safe Contract Interactions
 *
 * Demonstrates:
 * - Generating TypeScript types from contract metadata
 * - Type-safe contract method calls
 * - IDE autocomplete for all contract methods
 * - Compile-time type checking
 * - Typed event interfaces
 */

import {
  deploy,
  getContract,
  Network,
  Signer,
  balance,
} from '../packages/sdk/src';

// Import generated types
// In a real project, you would run: glin-forge typegen
// This generates types in ./types/[ContractName].ts
// import type { Flipper, FlipperQueries, FlipperTransactions, FlipperEvents } from './types/flipper';

async function main() {
  console.log('🎯 Phase 3: Type-Safe Contract Interactions\n');
  console.log('='.repeat(60));

  // ========================================
  // Step 1: Generate Types
  // ========================================

  console.log('\n📝 Step 1: Generate TypeScript Types');
  console.log('-'.repeat(60));

  console.log(`
  First, generate types from your contract metadata:

    $ glin-forge typegen

  This reads metadata from target/ink/metadata.json and generates:
  - Full TypeScript interfaces
  - Separated query and transaction methods
  - Event type definitions
  - Constructor interfaces

  Generated file: ./types/flipper.ts
  `);

  // ========================================
  // Step 2: Using Generated Types
  // ========================================

  console.log('\n💡 Step 2: Using Generated Types in Your Code');
  console.log('-'.repeat(60));

  console.log(`
  Import and use the generated types:

  \`\`\`typescript
  import type {
    Flipper,
    FlipperQueries,
    FlipperTransactions,
    FlipperEvents
  } from './types/flipper';

  // Deploy with type-safe arguments
  const contract = await deploy({
    wasm: './target/ink/flipper.wasm',
    metadata: './target/ink/metadata.json',
    args: [true], // ✅ TypeScript knows this must be a boolean
    network: Network.Testnet,
    account: Signer.Alice,
  });

  // Query with full type safety
  const value: boolean = await contract.query('get');
  //    ^^^^^^^ TypeScript infers the return type!

  // Transaction with autocomplete
  await contract.call('flip');
  //                   ^^^^
  //                   IDE shows: 'flip' | 'get'
  \`\`\`
  `);

  // ========================================
  // Step 3: Practical Example
  // ========================================

  console.log('\n🔧 Step 3: Practical Type-Safe Example');
  console.log('-'.repeat(60));

  try {
    console.log('  Deploying Flipper contract with initial value = true...');

    const contract = await deploy({
      wasm: './target/ink/flipper.wasm',
      metadata: './target/ink/metadata.json',
      args: [true],
      network: Network.Testnet,
      account: Signer.Alice,
    });

    console.log('  ✓ Contract deployed!');
    console.log('    Address:', contract.address);

    // Query current value (type-safe!)
    console.log('\n  Querying current value...');
    const currentValue = await contract.query('get');
    console.log('    Current value:', currentValue);

    // Flip the value (type-safe transaction)
    console.log('\n  Flipping value...');
    const tx = await contract.call('flip');
    console.log('    Transaction hash:', tx.hash());

    const receipt = await tx.wait();
    console.log('    ✓ Finalized in block:', receipt.blockNumber);

    // Query new value
    const newValue = await contract.query('get');
    console.log('    New value:', newValue);

  } catch (error: any) {
    console.log('  ℹ️ ', error.message);
    console.log('      (This is a demo - contract files may not exist)');
  }

  // ========================================
  // Step 4: Type Safety Benefits
  // ========================================

  console.log('\n✨ Step 4: Type Safety Benefits');
  console.log('-'.repeat(60));

  console.log(`
  With generated types, you get:

  1. **IDE Autocomplete**
     - All contract methods show up in autocomplete
     - Method signatures with parameter names
     - JSDoc comments from contract metadata

  2. **Compile-Time Checking**
     - Wrong method names caught before runtime
     - Wrong parameter types prevented
     - Return types automatically inferred

  3. **Refactoring Safety**
     - Rename contract methods? TypeScript finds all usages
     - Change parameter types? Compiler shows all affected code

  4. **Self-Documenting Code**
     - Generated types serve as documentation
     - Event definitions show all emitted events
     - Query vs Transaction methods clearly separated
  `);

  // ========================================
  // Step 5: Advanced Features
  // ========================================

  console.log('\n🚀 Step 5: Advanced Type Features');
  console.log('-'.repeat(60));

  console.log(`
  **Custom Types**
  The generator handles all ink! types:
  - Structs → TypeScript interfaces
  - Enums → Discriminated unions
  - Option<T> → T | null
  - Result<T, E> → { Ok: T } | { Err: E }
  - Vec<T> → T[]
  - Arrays → TypeScript arrays or Uint8Array

  **Event Types**
  \`\`\`typescript
  interface FlippedEvent {
    old_value: boolean;  // Previous value
    new_value: boolean;  // New value
  }

  contract.on('Flipped', (event: FlippedEvent) => {
    console.log(\`Flipped: \${event.old_value} → \${event.new_value}\`);
  });
  \`\`\`

  **Legacy Mode**
  For simpler projects, use --legacy flag:

    $ glin-forge typegen --legacy

  This generates basic interfaces without full type safety.
  `);

  // ========================================
  // Step 6: Workflow Integration
  // ========================================

  console.log('\n📦 Step 6: Workflow Integration');
  console.log('-'.repeat(60));

  console.log(`
  **Recommended Workflow:**

  1. Write and compile your ink! contract:
     $ cargo contract build

  2. Generate TypeScript types:
     $ glin-forge typegen

  3. Import and use types in your frontend:
     import type { MyContract } from './types/MyContract';

  4. Regenerate types after contract changes:
     $ glin-forge typegen

  **CI/CD Integration:**
  Add to your build pipeline:

  \`\`\`yaml
  - name: Build contract
    run: cargo contract build

  - name: Generate types
    run: glin-forge typegen --output src/types

  - name: Type check frontend
    run: npm run typecheck
  \`\`\`
  `);

  // ========================================
  // Summary
  // ========================================

  console.log('\n' + '='.repeat(60));
  console.log('✅ Phase 3: Type Generation Complete!');
  console.log('='.repeat(60));

  console.log('\nPhase 3 Features:');
  console.log('  ✓ TypeScript generation from ink! metadata');
  console.log('  ✓ Full type safety with IDE autocomplete');
  console.log('  ✓ Compile-time error detection');
  console.log('  ✓ Separated query/transaction interfaces');
  console.log('  ✓ Event type definitions');
  console.log('  ✓ Constructor interfaces');
  console.log('  ✓ Custom type mapping (structs, enums, etc.)');
  console.log('  ✓ Legacy mode support');

  console.log('\nNext Steps:');
  console.log('  1. Run: glin-forge typegen');
  console.log('  2. Import generated types in your code');
  console.log('  3. Enjoy full type safety and autocomplete!');

  console.log('\n📚 Documentation:');
  console.log('  Run: glin-forge typegen --help');
}

main()
  .then(() => {
    console.log('\n✨ Phase 3 demo complete!');
    process.exit(0);
  })
  .catch((error) => {
    console.error('\n❌ Error:', error.message);
    process.exit(1);
  });
