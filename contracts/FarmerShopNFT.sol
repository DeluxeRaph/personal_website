// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/// @title FarmerShopNFT
/// @notice Minimal ERC-721-style NFT for the 0xFarmer Agent Shop. Payments are handled off-chain via x402; this contract enforces max supply, endpoint-level cooldown, and minter-only minting.
contract FarmerShopNFT {
    string public name = "0xFarmer Agent Shop";
    string public symbol = "FARMER";
    uint256 public constant MAX_SUPPLY = 100;
    uint256 public constant COOLDOWN = 5 minutes;

    address public owner;
    address public minter;
    uint256 public totalSupply;

    mapping(uint256 => address) private _owners;
    mapping(address => uint256) private _balances;
    mapping(uint256 => address) private _tokenApprovals;
    mapping(address => mapping(address => bool)) private _operatorApprovals;
    mapping(uint256 => string) private _tokenURIs;
    uint256 public lastMintedAt;

    event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);
    event Approval(address indexed owner, address indexed approved, uint256 indexed tokenId);
    event ApprovalForAll(address indexed owner, address indexed operator, bool approved);
    event MinterUpdated(address indexed minter);
    event TokenURIUpdated(uint256 indexed tokenId, string tokenURI);

    modifier onlyOwner() {
        require(msg.sender == owner, "ONLY_OWNER");
        _;
    }

    modifier onlyMinter() {
        require(msg.sender == minter || msg.sender == owner, "ONLY_MINTER");
        _;
    }

    constructor(address initialMinter) {
        owner = msg.sender;
        minter = initialMinter == address(0) ? msg.sender : initialMinter;
        emit MinterUpdated(minter);
    }

    function setMinter(address newMinter) external onlyOwner {
        require(newMinter != address(0), "ZERO_MINTER");
        minter = newMinter;
        emit MinterUpdated(newMinter);
    }

    function mintTo(address to, string calldata uri) external onlyMinter returns (uint256 tokenId) {
        require(to != address(0), "ZERO_TO");
        require(totalSupply < MAX_SUPPLY, "SOLD_OUT");
        require(block.timestamp >= lastMintedAt + COOLDOWN, "COOLDOWN");
        require(bytes(uri).length >= 7 && _startsWithIpfs(uri), "TOKEN_URI_NOT_IPFS");

        tokenId = ++totalSupply;
        lastMintedAt = block.timestamp;
        _balances[to] += 1;
        _owners[tokenId] = to;
        _tokenURIs[tokenId] = uri;

        emit Transfer(address(0), to, tokenId);
        emit TokenURIUpdated(tokenId, uri);
    }

    function balanceOf(address account) external view returns (uint256) {
        require(account != address(0), "ZERO_ACCOUNT");
        return _balances[account];
    }

    function ownerOf(uint256 tokenId) public view returns (address) {
        address tokenOwner = _owners[tokenId];
        require(tokenOwner != address(0), "NOT_MINTED");
        return tokenOwner;
    }

    function tokenURI(uint256 tokenId) external view returns (string memory) {
        ownerOf(tokenId);
        return _tokenURIs[tokenId];
    }

    function approve(address to, uint256 tokenId) external {
        address tokenOwner = ownerOf(tokenId);
        require(msg.sender == tokenOwner || isApprovedForAll(tokenOwner, msg.sender), "NOT_AUTHORIZED");
        _tokenApprovals[tokenId] = to;
        emit Approval(tokenOwner, to, tokenId);
    }

    function getApproved(uint256 tokenId) public view returns (address) {
        ownerOf(tokenId);
        return _tokenApprovals[tokenId];
    }

    function setApprovalForAll(address operator, bool approved) external {
        require(operator != msg.sender, "SELF_APPROVAL");
        _operatorApprovals[msg.sender][operator] = approved;
        emit ApprovalForAll(msg.sender, operator, approved);
    }

    function isApprovedForAll(address tokenOwner, address operator) public view returns (bool) {
        return _operatorApprovals[tokenOwner][operator];
    }

    function transferFrom(address from, address to, uint256 tokenId) public {
        require(_isApprovedOrOwner(msg.sender, tokenId), "NOT_AUTHORIZED");
        require(ownerOf(tokenId) == from, "WRONG_FROM");
        require(to != address(0), "ZERO_TO");

        delete _tokenApprovals[tokenId];
        _balances[from] -= 1;
        _balances[to] += 1;
        _owners[tokenId] = to;
        emit Transfer(from, to, tokenId);
    }

    function safeTransferFrom(address from, address to, uint256 tokenId) external {
        transferFrom(from, to, tokenId);
    }

    function safeTransferFrom(address from, address to, uint256 tokenId, bytes calldata) external {
        transferFrom(from, to, tokenId);
    }

    function supportsInterface(bytes4 interfaceId) external pure returns (bool) {
        return interfaceId == 0x80ac58cd || interfaceId == 0x5b5e139f || interfaceId == 0x01ffc9a7;
    }

    function _isApprovedOrOwner(address spender, uint256 tokenId) internal view returns (bool) {
        address tokenOwner = ownerOf(tokenId);
        return spender == tokenOwner || getApproved(tokenId) == spender || isApprovedForAll(tokenOwner, spender);
    }

    function _startsWithIpfs(string calldata uri) internal pure returns (bool) {
        bytes calldata value = bytes(uri);
        return value.length >= 7
            && value[0] == "i"
            && value[1] == "p"
            && value[2] == "f"
            && value[3] == "s"
            && value[4] == ":"
            && value[5] == "/"
            && value[6] == "/";
    }
}
