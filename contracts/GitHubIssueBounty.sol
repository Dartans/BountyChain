// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract GitHubIssueBounty {
    struct Bounty {
        address creator;
        uint256 amount;
        bool approved;
        bool claimed;
    }

    mapping(string => Bounty) public bounties; // Maps GitHub issue URLs to bounties
    mapping(string => address) public issueFixers; // Maps issue URLs to fixers' wallet addresses
    mapping(string => bool) public issueClosed; // Tracks if an issue is marked closed
    mapping(address => bool) public maintainers; // Maintainers who can approve bounties
    mapping(string => string) public maintainerFiles; // Maintainer file tracking approval and fix link

    address public owner;

    event BountyPosted(string issueUrl, address creator, uint256 amount);
    event BountyIncreased(string issueUrl, uint256 newAmount);
    event BountyApproved(string issueUrl);
    event BountyClaimed(string issueUrl, address fixer, uint256 amount);
    event IssueClosed(string issueUrl);
    event MaintainerAdded(address maintainer);
    event MaintainerRemoved(address maintainer);
    event MaintainerFileUpdated(string issueUrl, string fileLink);

    modifier onlyOwner() {
        require(msg.sender == owner, "Not the contract owner");
        _;
    }

    modifier onlyMaintainer() {
        require(maintainers[msg.sender], "Not a maintainer");
        _;
    }

    constructor() {
        owner = msg.sender;
    }

    function addMaintainer(address maintainer) external onlyOwner {
        maintainers[maintainer] = true;
        emit MaintainerAdded(maintainer);
    }

    function removeMaintainer(address maintainer) external onlyOwner {
        maintainers[maintainer] = false;
        emit MaintainerRemoved(maintainer);
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

    function approveBounty(string memory issueUrl) external onlyMaintainer {
        require(bounties[issueUrl].amount > 0, "Bounty does not exist");
        require(!bounties[issueUrl].approved, "Bounty already approved");
        
        bounties[issueUrl].approved = true;
        emit BountyApproved(issueUrl);
    }

    function closeIssue(string memory issueUrl) external onlyMaintainer {
        require(bounties[issueUrl].amount > 0, "Bounty does not exist");
        issueClosed[issueUrl] = true;
        emit IssueClosed(issueUrl);
    }

    function updateMaintainerFile(string memory issueUrl, string memory fileLink) external onlyMaintainer {
        maintainerFiles[issueUrl] = fileLink;
        emit MaintainerFileUpdated(issueUrl, fileLink);
    }

    function claimBounty(string memory issueUrl) external {
        require(bounties[issueUrl].approved, "Bounty not approved");
        require(issueClosed[issueUrl], "Issue not closed");
        require(!bounties[issueUrl].claimed, "Bounty already claimed");
        require(issueFixers[issueUrl] == msg.sender, "Not the fix submitter");
        require(bytes(maintainerFiles[issueUrl]).length > 0, "Maintainer file not found");
        
        uint256 amount = bounties[issueUrl].amount;
        bounties[issueUrl].claimed = true;
        
        payable(msg.sender).transfer(amount);
        emit BountyClaimed(issueUrl, msg.sender, amount);
    }

    function setIssueFixer(string memory issueUrl, address fixer) external onlyMaintainer {
        issueFixers[issueUrl] = fixer;
    }

    function getBounty(string memory issueUrl) external view returns (uint256, bool, bool) {
        return (bounties[issueUrl].amount, bounties[issueUrl].approved, bounties[issueUrl].claimed);
    }
}
