import "std.sol";

/* TODO:
 *    should only be called by WhuffieAPI contracts
 *    should allow for updating WhuffieAPI address
 *    should be mortal
 *    should be stored in some kind of NameReg
 */
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
contract WhuffieStorage is activable, RestrictedAPI {
  AccountMap public Graph;  /**< The core mapping of Accounts and Offers */
  uint constant MAX_UINT = 2**256 - 1;

  function WhuffieStorage () {}

  /********************************************************//**
   * @struct AccountMap
   * @notice A doubly-linked list containing all Whuffie Accounts and Offers
   ***********************************************************/
  struct AccountMap {
    uint    size;           /**< length of the linked-list */
    address headAddr;       /**< source address of first Account of linked-list */
    address tailAddr;       /**< source address of last Account of linked-list */
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
   * @notice Creates a new Account in Graph
   * @param source Account's address
   * @param metadata IPFS hash of the account creation transaction
   */
  function createAccount(
    address source,
    string metadata
  ) public onlyAPI onlyActive returns (bool success) {
    if (accountExists(source)) { throw; }
    var size = Graph.size;
    if (size == MAX_UINT) { throw; }
    var _newAccount = Account(true, 0x0, 0x0, source, metadata, OfferMap(0, 0x0, 0x0));

    if (size == 0) {
      Graph.headAddr = source;
    } else {
      address oldTail = Graph.tailAddr;
      _setAccountNextAddr(oldTail, source);
      _newAccount.prevAddr = oldTail;
    }

    Graph.tailAddr = source;
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
  ) public onlyAPI onlyActive returns (bool success) {}

  /********************************************************//**
   * @struct Account
   * @notice A Whuffie-holding account
   ***********************************************************/
  struct Account {
    bool      exists;       /**< whether or not the Account exists */
    address   prevAddr;     /**< source address of previous Account in linked-list */
    address   nextAddr;     /**< source address of next Account in linked-list */

    address   sourceAddr;   /**< the Account's address */
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
  ) public onlyAPI onlyActive returns (bool success) {
    if (accountExists(source) == false) { throw; }
    Graph.accounts[source].metadata = metadata;
    return true;
  }

  function _setAccountPrevAddr(
    address source,
    address prevAddr
  ) internal returns (bool success) {
    Graph.accounts[source].prevAddr = prevAddr;
    return true;
  }

  function _setAccountNextAddr(
    address source,
    address nextAddr
  ) internal returns (bool success) {
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
    uint    size;         /**< length of the linked-list */
    address headAddr;     /**< source address of first Offer of linked-list */
    address tailAddr;     /**< source address of last Offer of linked-list */
    mapping (
      address => Offer    /**< hashmap of Offers by target address */
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
  ) public onlyAPI onlyActive returns (bool success) {
    if (offerExists(source, target)) { throw; }
    var size = getOfferMapSize(source);
    if (size == MAX_UINT) { throw; }

    var _newOffer = Offer(true, 0x0, 0x0, target, true, limit, exchangeRate, 0, 0, 0, 0);

    if (size == 0) {
      Graph.accounts[source].offerMap.headAddr = target;
    } else {
      address oldTail = Graph.accounts[source].offerMap.tailAddr;
      _setOfferNextAddr(source, oldTail, target);
      _newOffer.prevAddr = oldTail;
    }

    Graph.accounts[source].offerMap.tailAddr = target;
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
  ) public onlyAPI onlyActive returns (bool success) {}

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
    Graph.accounts[source].offerMap.offers[target].prevAddr = prevAddr;
    return true;
  }

  function _setOfferNextAddr(
    address source,
    address target,
    address nextAddr
  ) internal returns (bool success) {
    Graph.accounts[source].offerMap.offers[target].nextAddr = nextAddr;
    return true;
  }

  /**
   * @notice Determines if a Offer is alive
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
  ) public onlyAPI onlyActive returns (bool success) {
    if (offerExists(source, target) == false) { throw; }
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
   * @param newLimit The new limit of target credit to hold
   * @return success
   */
  function setOfferLimit(
    address source,
    address target,
    uint newLimit
  ) public onlyAPI onlyActive returns (bool success) {
    if (offerExists(source, target) == false) { throw; }
    Graph.accounts[source].offerMap.offers[target].limit = newLimit;
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
    uint newSourceBalance
  ) public onlyAPI onlyActive returns (bool success) {
    if (offerExists(source, target) == false) { throw; }
    Graph.accounts[source].offerMap.offers[target].sourceBalance = newSourceBalance;
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
    uint newTargetBalance
  ) public onlyAPI onlyActive returns (bool success) {
    if (offerExists(source, target) == false) { throw; }
    Graph.accounts[source].offerMap.offers[target].targetBalance = newTargetBalance;
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
    uint[2] newExchangeRate
  ) public onlyAPI onlyActive returns (bool success) {
    if (offerExists(source, target) == false) { throw; }
    Graph.accounts[source].offerMap.offers[target].exchangeRate = newExchangeRate;
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
    uint newSourceFrozenBalance
  ) public onlyAPI onlyActive returns (bool success) {
    if (offerExists(source, target) == false) { throw; }
    Graph.accounts[source].offerMap.offers[target].sourceFrozenBalance = newSourceFrozenBalance;
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
    uint newTargetFrozenBalance
  ) public onlyAPI onlyActive returns (bool success) {
    if (offerExists(source, target) == false) { throw; }
    Graph.accounts[source].offerMap.offers[target].targetFrozenBalance = newTargetFrozenBalance;
    return true;
  }

  // fallback function
  function() { throw; }
}
