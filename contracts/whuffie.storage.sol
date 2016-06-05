import "assertive.sol";
import "activable.sol";
import "restrictedAPI.sol";

// TODO: should be stored in some kind of NameReg
// TODO: audit and test all functions for redundancy, performance and "throw"-related errors

/**
 * @title WhuffieStorage
 * @author Sunny Gonnabathula | @sunny-g | sunny.gonna@gmail.com
 * @notice Implements public getters, setters and iterators for the Whuffie Graph
 * @dev This contract will maintain the base-level storage of all Accounts and Offers,
 *  and will only be mutated by selected API contracts. It is implemented under
 *  the assumption that contract storage cannot be migrated to a new contract.
 *  If this is untrue, then this contract can be updated to have higher-level
 *  functionality baked-in.
 */
contract WhuffieStorage is Assertive, Activable, RestrictedAPI {
  AccountMap public Graph;  /**< The core mapping of Accounts and Offers */
  uint constant MAX_UINT = 2**256 - 1;

  function WhuffieStorage () {}

  /********************************************************//**
   * @struct AccountMap
   * @notice A doubly-linked list containing all Whuffie Accounts and Offers
   ***********************************************************/
  struct AccountMap {
    uint    size;           /**< length of the linked-list */
    address firstAddr;      /**< source address of first Account of linked-list */
    address lastAddr;       /**< source address of last Account of linked-list */
    mapping (
      address => Account    /**< hashmap of Accounts by their address */
    ) accounts;
  }

  /**
   * @notice Internal method for fetching a Account struct from storage
   * @param source Address of desired account
   * @return Account instance
   */
  function _getAccount(
    address source
  ) internal constant returns (Account sourceAccount) {
    return Graph.accounts[source];
  }

  /**
   * @notice Returns Account struct members for a given address
   * @dev Must return individual members (since solidity doesn't allow struct
   *  return values within the EVM)
   * @param source Address of the account
   * @return exists
   * @return metadata IPFS hash of the account's last transaction
   */
  function getAccount(
    address source
  ) public constant returns (
    bool exists,
    string metadata
  ) {
    var _account = _getAccount(source);
    exists = _account.exists;
    metadata = _account.metadata;
  }

  /**
   * @notice Creates a new Account in the Graph
   * @param source Account's address
   * @param name Name of the Account's own credit
   * @param symbol Symbol associated with the Account's own credit
   * @param metadata IPFS hash of the account creation transaction
   */
  function createAccount(
    address source,
    string name,
    string symbol,
    string metadata
  ) public onlyAPI onlyActivated returns (bool success) {
    assert(accountExists(source));
    var size = Graph.size;
    assert(size == MAX_UINT);
    var _newAccount = Account(true, 0x0, 0x0, source, name, symbol, metadata, OfferMap(0, 0x0, 0x0));

    if (size == 0) {
      Graph.firstAddr = source;
    } else {
      address oldTailAddr = Graph.lastAddr;
      _setAccountNextAddr(oldTailAddr, source);
      _newAccount.prevAddr = oldTailAddr;
    }

    Graph.lastAddr = source;
    Graph.size = size + 1;
    Graph.accounts[source] = _newAccount;
    return true;
  }

  // internal helpers
  // TODO: audit these functions for redundancy, performance and "throw"-related errors
  /**
   * @notice Swaps two Account's positions within an AccountMap
   * @param sourceOne Address of source account
   * @param sourceTwo Address of second Account
   * @return bool
   */
  function swapAccounts(
    address sourceOne,
    address sourceTwo
  ) public onlyAPI onlyActivated returns (bool success) {
    assert(accountExists(sourceOne));
    assert(accountExists(sourceTwo));
  }

  /********************************************************//**
   * @struct Account
   * @notice A Whuffie-holding account
   ***********************************************************/
  struct Account {
    bool      exists;       /**< whether or not the Account exists */
    address   prevAddr;     /**< source address of previous Account in linked-list */
    address   nextAddr;     /**< source address of next Account in linked-list */

    address   sourceAddr;   /**< the Account's address */
    string    name;         /**< name of the Account's own credit */
    string    symbol;       /**< symbol for the Account's own credit */
    string    metadata;     /**< metadata regarding the Account's last transaction */
    OfferMap  offerMap;     /**< a collection of the Account's open offers */
  }

  /**
   * @notice Determines if the Account exists
   * @param source Account's address
   * @return bool
   */
  function accountExists(
    address source
  ) public constant returns (bool success) {
    return _getAccount(source).exists;
  }

  /**
   * @notice Fetches the name of the Account's credit
   * @param source Account's address
   * @return name Name of the Account's credit
   */
  function getName(
    address source
  ) public constant returns (string name) {
    return _getAccount(source).name;
  }

  /**
   * @notice Sets the name of the Account's credit
   * @param source Account's address
   * @param name New name of the Account's credit
   * @return bool
   */
  function setName(
    address source,
    string name
  ) public onlyAPI onlyActivated returns (bool success) {
    assert(accountExists(source));

    Graph.accounts[source].name = name;
    return true;
  }

  /**
   * @notice Fetches the symbol of the Account's credit
   * @param source Account's address
   * @return symbol Symbol of the Account's credit
   */
  function getSymbol(
    address source
  ) public constant returns (string symbol) {
    return _getAccount(source).symbol;
  }

  /**
   * @notice Sets symbol for the Account's credit
   * @param source Account's address
   * @param symbol New symbol for the Account's credit
   * @return bool
   */
  function setSymbol(
    address source,
    string symbol
  ) public onlyAPI onlyActivated returns (bool success) {
    assert(accountExists(source));

    Graph.accounts[source].symbol = symbol;
    return true;
  }

  /**
   * @notice Fetches the latest IPFS hash for a account
   * @param source Account's address
   * @return metadata IPFS hash of account's latest transaction
   */
  function getMetadata(
    address source
  ) public constant returns (string metadata) {
    return _getAccount(source).metadata;
  }

  /**
   * @notice Sets latest IPFS hash for a account
   * @param source Account's address
   * @param metadata IPFS hash of account's latest transaction
   * @return bool
   */
  function setMetadata(
    address source,
    string metadata
  ) public onlyAPI onlyActivated returns (bool success) {
    assert(accountExists(source));

    Graph.accounts[source].metadata = metadata;
    return true;
  }

  function _setAccountPrevAddr(
    address source,
    address prevAddr
  ) internal returns (bool success) {
    assert(accountExists(source));

    Graph.accounts[source].prevAddr = prevAddr;
    return true;
  }

  function _setAccountNextAddr(
    address source,
    address nextAddr
  ) internal returns (bool success) {
    assert(accountExists(source));

    Graph.accounts[source].nextAddr = nextAddr;
    return true;
  }

  /********************************************************//**
   * @struct OfferMap
   * @notice A doubly-linked list containing all of an Account's open Offers,
   *  sorted by ???
   * @dev O(1) get, add, remove, swap
   ***********************************************************/
  struct OfferMap {
    uint    size;           /**< length of the linked-list */
    address firstAddr;      /**< source address of first Offer of linked-list */
    address lastAddr;       /**< source address of last Offer of linked-list */
    mapping (
      address => Offer      /**< hashmap of Offers by target address */
    ) offers;
  }

  /**
   * @notice Internal method for fetching an individual Offer
   * @param source Address of source account
   * @param target Address of counterparty account
   * @return Offer instance
   */
  function _getOffer(
    address source,
    address target
  ) internal constant returns (Offer) {
    return Graph.accounts[source].offerMap.offers[target];
  }

  /**
   * @notice Returns Offer struct members
   * @dev Must return individual members (since solidity doesn't allow struct
   *  return values within the EVM)
   * @param source Address of source account
   * @param target Address of counterparty account
   * @return exists
   * @return prev
   * @return next
   * @return limit
   * @return exchangeRate
   * @return sourceBalance
   * @return targetBalance
   * @return frozenSourceBalance
   * @return frozenTargetBalance
   */
  function getOffer(
    address source,
    address target
  ) public constant returns (
    bool exists,
    address prevAddr,
    address nextAddr,
    uint limit,
    uint[2] exchangeRate,
    uint sourceBalance,
    uint targetBalance,
    uint sourceFrozenBalance,
    uint targetFrozenBalance
  ) {
    var _offer              = _getOffer(source, target);
    exists                  = _offer.exists;
    prevAddr                = _offer.prevAddr;
    nextAddr                = _offer.nextAddr;
    limit                   = _offer.limit;
    exchangeRate            = _offer.exchangeRate;
    sourceBalance           = _offer.sourceBalance;
    targetBalance           = _offer.targetBalance;
    sourceFrozenBalance     = _offer.sourceFrozenBalance;
    targetFrozenBalance     = _offer.targetFrozenBalance;
  }

  function _createOffer(
    address source,
    address target,
    Offer offer
  ) internal returns (bool success) {
    Graph.accounts[source].offerMap.offers[target] = offer;
    return true;
  }

  /**
   * @notice Creates a new Offer in the source account's OfferMap
   * @param source Address of source account
   * @param target Address of counterparty account
   * @return bool
   */
  function createOffer(
    address source,
    address target,
    uint limit,
    uint[2] exchangeRate
  ) public onlyAPI onlyActivated returns (bool success) {
    assert(offerExists(source, target));
    var size = getOfferMapSize(source);
    assert(size == MAX_UINT);

    var _newOffer = Offer(true, 0x0, 0x0, target, true, limit, exchangeRate, 0, 0, 0, 0);

    if (size == 0) {
      Graph.accounts[source].offerMap.firstAddr = target;
    } else {
      address oldTailAddr = Graph.accounts[source].offerMap.lastAddr;
      _setOfferNextAddr(source, oldTailAddr, target);
      _newOffer.prevAddr = oldTailAddr;
    }

    Graph.accounts[source].offerMap.lastAddr = target;
    Graph.accounts[source].offerMap.size = size + 1;
    Graph.accounts[source].offerMap.offers[target] = _newOffer;
    return true;
  }

  // internal helpers
  // TODO: audit these functions for redundancy, performance and "throw"-related errors
  /**
   * @notice Fetches the length of the OfferMap
   * @param source The source account's OfferMap
   * @return uint Size of the OfferMap
   */
  function getOfferMapSize(
    address source
  ) public constant returns (uint size) {
    return Graph.accounts[source].offerMap.size;
  }

  /**
   * @notice Swaps two Offers' positions within an OfferMap
   * @param source Address of source account
   * @param targetOne Address of first Offer
   * @param targetTwo Address of second Offer
   * @return bool
   */
  function swapOffers(
    address source,
    address targetOne,
    address targetTwo
  ) public onlyAPI onlyActivated returns (bool success) {
    assert(offerExists(source, targetOne));
    assert(offerExists(source, targetTwo));
  }

  /********************************************************//**
   * @struct Offer
   * @notice An offer to exchange the source's credits for the target's
   ***********************************************************/
  struct Offer {
    bool    exists;               /**< whether or not an Offer has been created */
    address prevAddr;             /**< target address of previous Offer in linked-list */
    address nextAddr;             /**< target address of next Offer's in linked-list */

    address targetAddr;           /**< address of Offer target */
    bool    active;               /**< whether or not the Offer can be used in transactions */
    uint    limit;                /**< maximum amount of target credit to hold */
    // TODO: fix this to be more ERC20-compliant
    uint[2] exchangeRate;         /**< exchange rate between target's and source's credit */
    uint    sourceBalance;        /**< balance of source's credit */
    uint    targetBalance;        /**< balance of target's credit */
    uint    sourceFrozenBalance;  /**< immovable balance of source's credit */
    uint    targetFrozenBalance;  /**< immovable balance of target's credit */
  }

  /**
   * @notice Determines if a Offer has ever been created
   * @param source Address of source account
   * @param target Address of counterparty account
   * @return bool
   */
  function offerExists(
    address source,
    address target
  ) public constant returns (bool exists) {
    return _getOffer(source, target).exists;
  }

  function _setOfferPrevAddr(
    address source,
    address target,
    address prevAddr
  ) internal returns (bool success) {
    assert(offerExists(source, target));

    Graph.accounts[source].offerMap.offers[target].prevAddr = prevAddr;
    return true;
  }

  function _setOfferNextAddr(
    address source,
    address target,
    address nextAddr
  ) internal returns (bool success) {
    assert(offerExists(source, target));

    Graph.accounts[source].offerMap.offers[target].nextAddr = nextAddr;
    return true;
  }

  /**
   * @notice Determines if a Offer is alive and usable for trades
   * @param source Address of source account
   * @param target Address of counterparty account
   * @return bool
   */
  function offerIsActive(
    address source,
    address target
  ) public constant returns (bool active) {
    return _getOffer(source, target).active;
  }

  /**
   * @param source Address of the offer owner
   * @param target Address of the offer's counterparty
   * @param activeStatus The new activity status of the offer
   * @return success
   */
  function setOfferActiveStatus(
    address source,
    address target,
    bool activeStatus
  ) public onlyAPI onlyActivated returns (bool success) {
    assert(offerExists(source, target));

    Graph.accounts[source].offerMap.offers[target].active = activeStatus;
    return true;
  }

  /**
   * @param source Address of the offer owner
   * @param target Address of the offer's counterparty
   * @return limit Maximum amount of target credit to hold
   */
  function getOfferLimit(
    address source,
    address target
  ) public constant returns (uint limit) {
    return _getOffer(source, target).limit;
  }

  /**
   * @param source Address of the offer owner
   * @param target Address of the offer's counterparty
   * @param limit The new limit of target credit to hold
   * @return success
   */
  function setOfferLimit(
    address source,
    address target,
    uint limit
  ) public onlyAPI onlyActivated returns (bool success) {
    assert(offerExists(source, target));

    Graph.accounts[source].offerMap.offers[target].limit = limit;
    return true;
  }

  function getOfferSourceBalance(
    address source,
    address target
  ) public constant returns (uint sourceBalancec) {
    return _getOffer(source, target).sourceBalance;
  }
  function setOfferSourceBalance(
    address source,
    address target,
    uint sourceBalance
  ) public onlyAPI onlyActivated returns (bool success) {
    assert(offerExists(source, target));

    Graph.accounts[source].offerMap.offers[target].sourceBalance = sourceBalance;
    return true;
  }

  function getOfferTargetBalance(
    address source,
    address target
  ) public constant returns (uint targetBalancec) {
    return _getOffer(source, target).targetBalance;
  }
  function setOfferTargetBalance(
    address source,
    address target,
    uint targetBalance
  ) public onlyAPI onlyActivated returns (bool success) {
    assert(offerExists(source, target));

    Graph.accounts[source].offerMap.offers[target].targetBalance = targetBalance;
    return true;
  }

  function getOfferExchangeRate(
    address source,
    address target
  ) public constant returns (uint[2] exchangeRate) {
    return _getOffer(source, target).exchangeRate;
  }

  function setOfferExchangeRate(
    address source,
    address target,
    uint[2] exchangeRate
  ) public onlyAPI onlyActivated returns (bool success) {
    assert(offerExists(source, target));

    Graph.accounts[source].offerMap.offers[target].exchangeRate = exchangeRate;
    return true;
  }

  function getOfferFrozenSourceBalance(
    address source,
    address target
  ) public constant returns (uint frozenSourceBalance) {
    return _getOffer(source, target).sourceFrozenBalance;
  }
  function setOfferFrozenSourceBalance(
    address source,
    address target,
    uint sourceFrozenBalance
  ) public onlyAPI onlyActivated returns (bool success) {
    assert(offerExists(source, target));

    Graph.accounts[source].offerMap.offers[target].sourceFrozenBalance = sourceFrozenBalance;
    return true;
  }

  function getOfferFrozenTargetBalance(
    address source,
    address target
  ) public constant returns (uint frozenTargetBalance) {
    return _getOffer(source, target).targetFrozenBalance;
  }
  function setOfferFrozenTargetBalance(
    address source,
    address target,
    uint targetFrozenBalance
  ) public onlyAPI onlyActivated returns (bool success) {
    assert(offerExists(source, target));

    Graph.accounts[source].offerMap.offers[target].targetFrozenBalance = targetFrozenBalance;
    return true;
  }

  // fallback function
  function() { throw; }
}
