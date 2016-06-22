import "dapple/test.sol";
import "whuffie.storage.sol";

contract BaseStorageTest is Test {
  WhuffieStorage whuffie;
  Tester proxy_tester;

  function setUp() {
    whuffie = new WhuffieStorage();
    whuffie.activate();
    whuffie.addAPI(address(this), 'test');
    proxy_tester = new Tester();
    proxy_tester._target(whuffie);
  }
}

contract StorageTest is BaseStorageTest {
  function test_is_Whuffie_owner() {
    assertEq(address(this), whuffie.owner());
  }
}