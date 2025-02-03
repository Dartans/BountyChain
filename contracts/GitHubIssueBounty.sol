// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract GitHubIssueBounty {
    struct Bounty {
        address creator;
        uint256 amount;
        bool claimed;
    }

    mapping(string => Bounty) public bounties; // Maps GitHub issue URLs to bounties
    mapping(string => string) public maintainerFiles; // Maintainer file tracking approval and fix link

    address public owner;

    event BountyPosted(string issueUrl, address creator, uint256 amount);
    event BountyIncreased(string issueUrl, uint256 newAmount);
    event BountyClaimed(string issueUrl, address fixer, uint256 amount);
    event MaintainerFileUpdated(string issueUrl, string fileLink);

    modifier onlyOwner() {
        require(msg.sender == owner, "Not the contract owner");
        _;
    }

    constructor() {
        owner = msg.sender;
    }

    function postBounty(string memory issueUrl) external payable {
        require(msg.value > 0, "Must send ETH for bounty");
        
        bounties[issueUrl].creator = msg.sender;
        bounties[issueUrl].amount += msg.value;
        
        emit BountyPosted(issueUrl, msg.sender, msg.value);
    }

    function increaseBounty(string memory issueUrl) external payable {
        require(msg.value > 0, "Must send ETH to increase bounty");
        require(bounties[issueUrl].amount > 0, "Bounty does not exist");

        bounties[issueUrl].amount += msg.value;
        emit BountyIncreased(issueUrl, bounties[issueUrl].amount);
    }

    function updateMaintainerFile(string memory issueUrl, string memory fileLink) external onlyOwner {
        maintainerFiles[issueUrl] = fileLink;
        emit MaintainerFileUpdated(issueUrl, fileLink);
    }

    function claimBounty(string memory issueUrl, address fixer) external {
        require(bytes(maintainerFiles[issueUrl]).length > 0, "Maintainer file not found");
        require(!bounties[issueUrl].claimed, "Bounty already claimed");
        
        uint256 amount = bounties[issueUrl].amount;
        bounties[issueUrl].claimed = true;
        
        payable(fixer).transfer(amount);
        emit BountyClaimed(issueUrl, fixer, amount);
    }

    function getBounty(string memory issueUrl) external view returns (uint256, bool) {
        return (bounties[issueUrl].amount, bounties[issueUrl].claimed);
    }
}
