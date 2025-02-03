const { ethers } = require("hardhat");

async function main() {
  const GitHubIssueBounty = await ethers.getContractFactory("GitHubIssueBounty");
  const bounty = await GitHubIssueBounty.deploy();

  await bounty.deployed();

  console.log("GitHubIssueBounty deployed to:", bounty.address);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
