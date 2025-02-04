#!/bin/bash

# Deployment script for Bounty Board program
# Usage: ./scripts/deploy.sh <network>

NETWORK=${1:-mainnet-beta}
PROGRAM_NAME="bounty_board"
ANCHOR_TEST_WALLET="$HOME/.config/solana/id.json"

echo "🔄 Building program..."
anchor build

echo "📦 Deploying to $NETWORK..."
anchor deploy --provider.cluster $NETWORK --provider.wallet $ANCHOR_TEST_WALLET --program-name $PROGRAM_NAME

echo "🛠️  Initializing bounty board..."
node scripts/sync-config.js

echo "✅ Deployment completed!"