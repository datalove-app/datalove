import "lib/whuffie.types.sol";

/**
 * @notice Implements
 */
library Accounts {
  /**
   * @notice Internal method for fetching a Account struct from storage
   * @param source Address of desired account
   * @return Account instance
   */
  function getAccount(
    Types.AccountMap storage Graph,
    address source
  ) constant returns (Types.Account storage sourceAccount) {
    return Graph.accounts[source];
  }

  /**
   * @notice Creates a new Account in the Graph
   * @param source Account's address
   * @param metadata IPFS hash of the account creation transaction
   */
  function createAccount(
    Types.AccountMap storage Graph,
    address source,
    address owner,
    bytes12 creditSymbol,
    bytes32 creditName,
    uint8   decimals,
    uint    initialTotalSupply,
    uint    initialSourceBalance,
    bytes32 metadata
  ) returns (bool success) {
    Graph.accounts[source] = Types.Account(
      metadata, source, creditSymbol, creditName,
      initialTotalSupply, initialSourceBalance, 0,
      decimals, true, owner, 0x0, 0x0,
      Types.CreditMap(0, 0, 0),
      Types.OfferMap(0, 0x0, 0x0)
    );

    Graph.size++;
    return true;
  }
}

// library Account {}
