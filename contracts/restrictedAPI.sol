import "std.sol";

contract RestrictedAPI is mortal {
  mapping (address => bool) public APIAccess;

  modifier onlyAPI() { if (APIAccess[msg.sender] == false) throw; _ }

  function addAPI(address addr) onlyOwner {
    APIAccess[addr] = true;
  }

  function revokeAPI(address addr) onlyOwner {
    APIAccess[addr] = false;
  }
}
