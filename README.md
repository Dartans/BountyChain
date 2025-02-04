# Bounty Board Chain

## Overview
Bounty Board Chain is a Solana-based smart contract ecosystem that facilitates transparent and trustless bounty management using Anchor. It allows developers to create, claim, and process bounties while ensuring secure fund allocation through escrow and decentralized governance.

## Features
- **Auto-Approval for Verified Contributors**: Developers with successful claims bypass manual approval.
- **Escrow-Based Bounty Funding**: Ensures payout integrity.
- **Token-Based Governance**: Tokens act as voting shares.
- **Minimal Maintainer Involvement**: GitHub-based automation.
- **Public Auditability**: Blockchain-verified transactions.

## File Structure
```
bounty-board-chain/
├── app/
│   ├── public/
│   ├── src/
│   │   ├── api/
│   │   │   └── webhook.ts
│   │   ├── components/
│   │   │   └── BountyList.tsx
│   │   ├── contexts/
│   │   │   └── SolanaProvider.tsx
│   │   └── App.tsx
├── programs/
│   └── bounty-board/
│       ├── src/
│       │   ├── lib.rs
│       │   ├── instructions/
│       │   │   ├── create_bounty.rs
│       │   │   ├── claim_bounty.rs
│       │   │   └── process_payout.rs
│       │   └── state.rs
│       ├── tests/
│       └── target/
├── migrations/
│   └── deploy.ts
├── .env
├── .gitignore
├── anchor.toml
├── package.json
└── tsconfig.json
```

## Smart Contract Implementation
### State Definitions
Located in `programs/bounty-board/src/state.rs`:
- `BountyBoard`: Manages bounties and payouts.
- `Bounty`: Represents an individual bounty.
- `BountyStatus`: Enum for tracking bounty lifecycle.

### Core Instructions
#### 1. Create Bounty (`create_bounty.rs`)
Handles escrow deposits and bounty creation.
#### 2. Process Payout (`process_payout.rs`)
Splits payouts securely among developers, public pools, and maintainers.

## Frontend Integration
Implemented in `app/src/components/BountyList.tsx`. Fetches bounty data via Solana RPC and listens for on-chain events.

## Deployment Guide
### Install Dependencies
```bash
yarn add @project-serum/anchor @solana/web3.js @solana/wallet-adapter-react
```
### Build & Deploy
```bash
anchor build
anchor deploy
```
### Initialize Board
```bash
ts-node migrations/deploy.ts
```

## Security & Best Practices
- **Anchor PDA-based Account Management**
- **Checked Arithmetic for Overflow Protection**
- **Secure Token Transfers via CPI**
- **WebSocket-Based Real-Time Updates**