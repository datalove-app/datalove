import "std.sol";

contract Activable is mortal {
  bool public activated;

  modifier onlyActivated() { if (activated == false) throw; _ }

  function activate() onlyOwner {
    activated = true;
  }

  function deactivate() onlyOwner {
    activated = false;
  }
}
