import "lib/std.sol";

contract RestrictedAPI is abstract, owned {
  mapping (address => bool) public APIAccess;
  mapping (bytes24 => address) public APIVersions;

  modifier onlyAPI() { if (APIAccess[msg.sender] == false) throw; _ }

  function addAPI(address addr, bytes24 version) onlyOwner returns (bool) {
    APIVersions[version] = addr;
    APIAccess[addr] = true;
    return true;
  }

  function revokeAPI(address addr) onlyOwner returns (bool) {
    APIAccess[addr] = false;
    return true;
  }
}
