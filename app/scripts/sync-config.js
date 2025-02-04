const fs = require('fs');
const { PublicKey } = require('@solana/web3.js');

// Sync program ID with frontend config
const programId = new PublicKey('BOUNTYBOARDCHAINPROGRAMID');
const configPath = './config/config.json';

const config = {
  programId: programId.toString(),
  network: process.env.ANCHOR_PROVIDER_URL || 'https://api.mainnet-beta.solana.com',
};

fs.writeFileSync(configPath, JSON.stringify(config, null, 2));
console.log('⚙️  Config file updated with program ID:', programId.toString());