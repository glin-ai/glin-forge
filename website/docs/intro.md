---
title: Introduction to GLIN Forge
description: The official CLI tool for building, deploying, and managing ink! smart contracts on GLIN Network
slug: /
---

# Welcome to GLIN Forge

GLIN Forge is the official command-line interface (CLI) tool for developing, deploying, and interacting with ink! smart contracts on the GLIN Network. Built for developers by developers, it provides a seamless experience from contract creation to production deployment.

## What is GLIN Forge?

Think of GLIN Forge as your complete toolkit for smart contract development on GLIN Network - similar to how Hardhat serves Ethereum or `gh` serves GitHub. It handles everything from scaffolding new contracts to deploying them on-chain and generating TypeScript types for your frontend.

## Key Features

### Smart Contract Development
- **Pre-built Templates** - Start quickly with battle-tested ERC20, ERC721, and DAO templates
- **Build & Compile** - Seamless integration with cargo-contract for optimal builds
- **Testing Support** - Run unit and end-to-end tests with ease
- **Contract Verification** - Verify your contracts on the block explorer automatically

### Deployment & Interaction
- **Multi-Network Support** - Deploy to testnet, mainnet, or local nodes with a single flag
- **Gas Optimization** - Built-in gas estimation with safety recommendations
- **Interactive Calls** - Query contract state or execute transactions with simple commands
- **Event Watching** - Monitor contract events in real-time or historically

### Developer Experience
- **TypeScript Generation** - Auto-generate fully-typed interfaces from contract ABIs
- **React Hooks** - Create ready-to-use React hooks for frontend integration
- **Account Management** - Generate, import, and manage accounts securely
- **Network Configuration** - Easily switch between networks and RPC endpoints

## Quick Example

Here's how easy it is to create and deploy a token contract:

```bash
# Create a new ERC20 token project
glin-forge new my-token --template erc20

# Build the contract
cd my-token
glin-forge build --release

# Deploy to testnet
glin-forge deploy --network testnet --account alice

# Query the total supply
glin-forge query 5ContractAddr... totalSupply

# Generate TypeScript types for your frontend
glin-forge typegen --output ./frontend/types --hooks
```

That's it! You now have a deployed token contract with type-safe frontend integration.

## Why GLIN Forge?

### Unified Workflow
Instead of juggling multiple tools, GLIN Forge provides a single CLI for your entire development lifecycle - from scaffolding to deployment to frontend integration.

### Type Safety
Automatically generate TypeScript types and React hooks from your contract ABIs, eliminating manual type definitions and reducing bugs.

### Network Flexibility
Seamlessly switch between testnet, mainnet, and local development networks without configuration headaches.

### Production Ready
Built on battle-tested libraries (subxt, ink!), GLIN Forge is production-ready from day one with gas optimization, verification, and security best practices built-in.

## Who is GLIN Forge For?

- **Smart Contract Developers** - Build and deploy contracts efficiently
- **Frontend Developers** - Integrate contracts with type-safe generated code
- **DevOps Engineers** - Automate deployments and verification
- **Contract Auditors** - Verify and analyze deployed contracts

## Getting Started

Ready to build your first contract? Check out our [Quick Start Guide](./getting-started/quick-start) to deploy your first contract in 5 minutes.

New to ink! or smart contracts? Start with [Prerequisites](./getting-started/prerequisites) to set up your development environment.

## Architecture Overview

GLIN Forge is built on a modular architecture:

- **CLI Layer** - User-friendly command interface with rich output
- **Contract Operations** - Core blockchain interaction via glin-sdk-rust
- **Code Generation** - TypeScript and React code generation from ABIs
- **Network Management** - Multi-network support with custom configurations
- **Template System** - Handlebars-based contract scaffolding

## Community & Support

- **Documentation** - You're reading it! Browse the sidebar for detailed guides
- **GitHub** - [github.com/glin-ai/glin-forge](https://github.com/glin-ai/glin-forge)
- **Discord** - [discord.gg/glin](https://discord.gg/glin)
- **Twitter** - [@glin_ai](https://twitter.com/glin_ai)

## Next Steps

<div className="next-steps">

- [Install GLIN Forge](./getting-started/installation) - Get the CLI installed on your system
- [Prerequisites](./getting-started/prerequisites) - Set up your development environment
- [Quick Start](./getting-started/quick-start) - Deploy your first contract in 5 minutes
- [First Contract](./getting-started/first-contract) - Step-by-step contract creation guide

</div>

---

Built with care by the GLIN team. Open source and free forever.
