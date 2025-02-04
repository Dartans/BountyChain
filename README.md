This project is under active development. see development plan file for full overview.

# GitHub Issue Bounty Smart Contract

This smart contract allows users to post bounties on GitHub issues, which can then be approved, claimed, and managed by maintainers.

## Features
- Post, increase, approve, and claim bounties
- Close issues and track maintainer files
- Add and remove maintainers

## Getting Started
To deploy and interact with the contract, follow these steps:

1. Clone the repository
2. Install dependencies
3. Compile and deploy the contract
4. Interact with the contract using a web3 provider

## Installation
Clone the repository:
```sh
git clone https://github.com/your-username/github-issue-bounty.git
cd github-issue-bounty
```
Install dependencies:
```sh
npm install
```
Compile the contract:
```sh
npx hardhat compile
```
Deploy the contract:
```sh
npx hardhat run scripts/deploy.js --network <network-name>
```
# GitHub Issue Bounty Smart Contract

## Overview
This smart contract enables a decentralized bounty system for unresolved GitHub issues. Users can contribute funds to encourage issue resolution, and funds are only released when the fix is verified through a structured repository-based approval process.

## Features
- **Post a Bounty:** Anyone can create a bounty by sending ETH to the contract for a specific GitHub issue.
- **Increase a Bounty:** Users can add additional ETH to an existing bounty.
- **Maintainer Approval via GitHub:** Maintainers approve bounties by updating a structured file in the repository under `bountyboard/`.
- **Automated Verification:** The contract verifies bounty conditions by checking structured GitHub repository files.
- **Claiming a Bounty:** Once an issue is closed and approved, the fix submitter can claim the bounty.

## Workflow
1. **Posting a Bounty:**
   - A user submits a bounty by calling `postBounty(issueUrl)` and sending ETH.
   - The contract stores the bounty amount and creator.

2. **Maintainers Approve Fixes:**
   - A project maintainer updates the repository under `bountyboard/` with a structured file containing:
     - Issue URL
     - Fix submitter's wallet address
     - Link to the merged fix

3. **Claiming the Bounty:**
   - The fix submitter verifies that their code is merged and executes `claimBounty(issueUrl, fixerAddress)`.
   - The contract checks the corresponding `bountyboard/` file for validation.
   - If conditions are met, the funds are released to the submitter.

## Deployment & Usage

### Deploying the Contract
1. Compile and deploy the contract using Remix or Hardhat.
2. Set the owner (deployer) who can manage maintainers.
3. Fund the contract to allow bounties to be created.

### Posting a Bounty
```solidity
postBounty("https://github.com/repo/issues/123")
```
- Must send ETH along with the transaction.

### Increasing a Bounty
```solidity
increaseBounty("https://github.com/repo/issues/123")
```
- Must send ETH along with the transaction.

### Claiming a Bounty
```solidity
claimBounty("https://github.com/repo/issues/123", msg.sender)
```
- The contract will verify the fix through the structured GitHub `bountyboard/` file.
- If valid, funds are released to the submitter.

## Repository Structure for Verification
```
/repository-root
│-- bountyboard/
│   │-- issue-123.json
│   │-- issue-456.json
```
Each `issue-XXX.json` file should contain:
```json
{
  "issueUrl": "https://github.com/repo/issues/123",
  "fixerWallet": "0x1234567890abcdef...",
  "fixLink": "https://github.com/repo/commit/abcdef"
}
```

## Security Considerations
- The contract relies on structured files in the repository to verify fixes.
- Only valid issue URLs should be used to prevent malicious claims.
- Ensure wallet addresses in the `bountyboard/` files are correct before submitting claims.

## License
This project is licensed under the MIT License.

![alt text](https://github.com/Dartans/BountyChain/blob/main/BountyChain.jpg)
