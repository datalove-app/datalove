import "lib/std.sol";

contract Assertive is abstract {
  function assert(bool assertion) internal {
    if (assertion == false) throw;
  }

  function throwIf(bool assertion) internal {
    if (assertion == true) throw;
  }
}
