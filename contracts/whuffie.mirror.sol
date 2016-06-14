/**
 * WhuffieTokenPeg
 * has two functions, deposit and withdraw
 * user should create it if it doesn't already exist within the WhuffieTokenRegistry
 *  this would map token addresses to WhuffieTokenPeg addresses
 *  should also maintain their names and symbols, preferably taken from the original token contract's storage
 */

// interface contract, used to call a token's ERC20 function's without having it's ABI
contract ERC20Token {
  function totalSupply() constant returns (uint256 supply);

  function balanceOf(address _owner) constant returns (uint256 balance);

  /// @notice send `_value` token to `_to` from `msg.sender`
  function transfer(address _to, uint256 _value) returns (bool success);

  /// @notice send `_value` token to `_to` from `_from` on the condition it is approved by `_from`
  function transferFrom(address _from, address _to, uint256 _value) returns (bool success);

  /// @notice `msg.sender` approves `_spender` to spend `_value` tokens
  /// @param _spender The address of the account able to transfer the tokens
  /// @param _value The amount of wei to be approved for transfer
  /// @return Whether the approval was successful or not
  function approve(address _spender, uint256 _value) returns (bool success);

  /// @param _owner The address of the account owning tokens
  /// @param _spender The address of the account able to transfer the tokens
  /// @return Amount of remaining tokens allowed to spent
  function allowance(address _owner, address _spender) constant returns (uint256 remaining);

  event Transfer(address indexed _from, address indexed _to, uint256 _value);
  event Approval(address indexed _owner, address indexed _spender, uint256 _value);
}

contract WhuffieTokenPeg {
  function WhuffieTokenPeg() {

  }
}
