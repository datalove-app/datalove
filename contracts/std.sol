// This si marker
// for contract not
// to be deployed to
// any environment
contract abstract {}

contract owned is abstract {
  address owner;

  function owned() {
    owner = msg.sender;
  }

  modifier onlyOwner() { if (msg.sender==owner) _ }

  function changeOwner(address newOwner) onlyOwner {
    owner = newOwner;
  }
}

contract mortal is abstract, owned {
  function kill() onlyOwner {
    if (msg.sender == owner) suicide(owner);
  }
}

contract NameReg is abstract {
  function register(bytes32 name) {}
  function unregister() {}
  function addressOf(bytes32 name) constant returns (address addr) {}
  function nameOf(address addr) constant returns (bytes32 name) {}
  function kill() {}
}

contract nameRegAware is abstract {
  function nameRegAddress() returns (address) {
    return 0x084f6a99003dae6d3906664fdbf43dd09930d0e3;
  }

  function named(bytes32 name) returns (address) {
    return NameReg(nameRegAddress()).addressOf(name);
  }
}

contract named is abstract, nameRegAware {
  function named(bytes32 name) {
    NameReg(nameRegAddress()).register(name);
  }
}

// contract with util functions
contract util is abstract {
  // Converts 'string' to 'bytes32'
  function s2b(string s) internal returns (bytes32) {
      bytes memory b = bytes(s);
      uint r = 0;
      for (uint i = 0; i < 32; i++) {
          if (i < b.length) {
              r = r | uint(b[i]);
          }
          if (i < 31) r = r * 256;
      }
      return bytes32(r);
  }
}
