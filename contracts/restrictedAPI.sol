import "std.sol";

contract RestrictedAPI is owned {
  mapping (address => bool) public APIAccess;
  mapping (string => address) public APIVersions;

  modifier onlyAPI() { if (APIAccess[msg.sender] == false) throw; _ }

  function addAPI(address addr, string version) onlyOwner {
    // TODO: check what a `null` address' hex value is, then test this
    if (APIVersions[version] != 0x0) throw;
    APIVersions[version] = addr;
    APIAccess[addr] = true;
  }

  function revokeAPI(address addr) onlyOwner {
    APIAccess[addr] = false;
  }
}
