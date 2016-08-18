import "lib/std.sol";

contract Activable is abstract, owned {
  bool public activated;

  modifier onlyActivated() { if (activated == false) throw; _ }

  function activate() onlyOwner returns (bool success) {
    activated = true;
    return true;
  }

  function deactivate() onlyOwner returns (bool success) {
    activated = false;
    return true;
  }
}
