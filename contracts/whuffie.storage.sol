/* TODO:
 *    should only be called by WhuffieAPI contracts
 *    should allow for updating WhuffieAPI address
 *    should be mortal
 *    should be stored in some kind of NameReg
 *
 *    should be IMMUTABLE (i.e. non-updateable, since that would require a state migration)
 *      this means that all methods and state mutations will be set
 *      in stone for the life of the contract
 */

/**
 * @title WhuffieStorage
 * @author Sunny Gonnabathula @sunny-g
 * @notice Implements public getters, setters and iterators for the Whuffie Graph
 * @dev This contract will maintain the base-level storage of all Users and Offers,
 *  and will only be mutated by selected API contracts. It is implemented under
 *  the assumption that contract storage cannot be migrated to a new contract.
 *  If this is untrue, then this contract can be update to have higher-level
 *  functionality baked-in.
 */
contract WhuffieStorage {
  mapping (address => bool) public APIAccess;   /**< All addresses allowed to mutate WhuffieStorage */
  UserMap public Graph;                         /**< The core mapping of Users and Offers */

  function WhuffieStorage (address APIaddr) {
    // set the initial API address for use in modifier
    APIAccess[APIaddr] = true;
  }

  modifier isAPI() { if (APIAccess[msg.sender]) _ }

  /********************************************************//**
   * @struct Offer
   * @notice An open offer tracking the exchange of the source's and target's credits
   ***********************************************************/
  struct Offer {
    bool    exists;               /**< whether or not an Offer has been created */
    address prev;                 /**< pointer to previous Offer of linked-list */
    address next;                 /**< pointer to next Offer of linked-list */

    address targetAddr;           /**< address of Offer target */
    bool    active;               /**< whether or not the Offer can be used in transactions */
    uint    limit;                /**< maximum amount of target credit to hold */
    uint[2] exchangeRate;         /**< exchange rate between target's and source's credit */
    uint    sourceBalance;        /**< balance of source's credit */
    uint    targetBalance;        /**< balance of target's credit */
    uint    sourceLockedBalance;  /**< immovable balance of source's credit */
    uint    targetLockedBalance;  /**< immovable balance of target's credit */
  }

  /**
   * @notice Determines if a Offer has ever been created
   * @param source Address of source user
   * @param target Address of counterparty user
   * @return bool
   */
  function offerExists(
    address source,
    address target
  ) public constant returns (bool) {
    return _getOffer(source, target).exists;
  }

  /**
   * @notice Determines if a Offer is alive
   * @param source Address of source user
   * @param target Address of counterparty user
   * @return bool
   */
  function offerIsActive(
    address source,
    address target
  ) public constant returns (bool) {
    return _getOffer(source, target).active;
  }

  /**
   * @param source Address of the offer owner
   * @param target Address of the offer's counterparty
   * @param activeStatus The new activity status of the offer
   * @return success
   */
  function setActiveStatus(
    address source,
    address target,
    bool activeStatus
  ) public returns (bool) {
    if (offerExists(source, target) == false) { throw; }
    Graph.users[source].offerMap.offers[target].active = activeStatus;
    return true;
  }

  /**
   * @param source Address of the offer owner
   * @param target Address of the offer's counterparty
   * @return limit Maximum amount of target credit to hold
   */
  function getLimit(
    address source,
    address target
  ) public constant returns (uint) {
    return _getOffer(source, target).limit;
  }

  /**
   * @param source Address of the offer owner
   * @param target Address of the offer's counterparty
   * @param newLimit The new limit of target credit to hold
   * @return success
   */
  function setLimit(
    address source,
    address target,
    uint newLimit
  ) public returns (bool) {
    if (offerExists(source, target) == false) { throw; }
    Graph.users[source].offerMap.offers[target].limit = newLimit;
    return true;
  }

  function getSourceBalance(
    address source,
    address target
  ) public constant returns (uint) {
    return _getOffer(source, target).sourceBalance;
  }
  function setSourceBalance(
    address source,
    address target,
    uint newSourceBalance
  ) public returns (bool) {
    if (offerExists(source, target) == false) { throw; }
    Graph.users[source].offerMap.offers[target].sourceBalance = newSourceBalance;
    return true;
  }

  function getTargetBalance(
    address source,
    address target
  ) public constant returns (uint) {
    return _getOffer(source, target).targetBalance;
  }
  function setTargetBalance(
    address source,
    address target,
    uint newTargetBalance
  ) public returns (bool) {
    if (offerExists(source, target) == false) { throw; }
    Graph.users[source].offerMap.offers[target].targetBalance = newTargetBalance;
    return true;
  }

  function getExchangeRate(
    address source,
    address target
  ) public constant returns (uint[2]) {
    return _getOffer(source, target).exchangeRate;
  }
  function setExchangeRate(
    address source,
    address target,
    uint[2] newExchangeRate
  ) public returns (bool) {
    if (offerExists(source, target) == false) { throw; }
    Graph.users[source].offerMap.offers[target].exchangeRate = newExchangeRate;
    return true;
  }

  function getLockedSourceBalance(
    address source,
    address target
  ) public constant returns (uint) {
    return _getOffer(source, target).sourceLockedBalance;
  }
  function setLockedSourceBalance(
    address source,
    address target,
    uint newSourceLockedBalance
  ) public returns (bool) {
    if (offerExists(source, target) == false) { throw; }
    Graph.users[source].offerMap.offers[target].sourceLockedBalance = newSourceLockedBalance;
    return true;
  }

  function getLockedTargetBalance(
    address source,
    address target
  ) public constant returns (uint lockedBalance) {
    return _getOffer(source, target).targetLockedBalance;
  }
  function setLockedTargetBalance(
    address source,
    address target,
    uint newTargetLockedBalance
  ) public returns (bool) {
    if (offerExists(source, target) == false) { throw; }
    Graph.users[source].offerMap.offers[target].targetLockedBalance = newTargetLockedBalance;
    return true;
  }

  /********************************************************//**
   * @struct OfferMap
   * @notice A doubly-linked list containing all of a User's open Offers,
   *  sorted by ??? (TODO: settle this when implementing swapOffers)
   * @dev O(1) get, add, remove, swap
   ***********************************************************/
  struct OfferMap {
    uint    size;         /**< length of the linked-list */
    address head;         /**< pointer to the first Offer of linked-list */
    address tail;         /**< pointer to the last Offer of linked-list */
    mapping (
      address => Offer   /**< hashmap of Offers by target address */
    ) offers;
  }

  /**
   * @notice Internal method for fetching an individual Offer
   * @param source Address of source user
   * @param target Address of counterparty user
   * @return Offer instance
   */
  function _getOffer(
    address source,
    address target
  ) internal constant returns (Offer) {
    return Graph.users[source].offerMap.offers[target];
  }

  /**
   * @notice Returns Offer struct members
   * @dev Must return individual members (since solidity doesn't allow struct
   *  return values within the EVM)
   * @param source Address of source user
   * @param target Address of counterparty user
   * @return exists
   * @return prev
   * @return next
   * @return limit
   * @return exchangeRate
   * @return sourceBalance
   * @return targetBalance
   * @return lockedSourceBalance
   * @return lockedTargetBalance
   */
  function getOffer(
    address source,
    address target
  ) public constant returns (
    bool exists,
    address prev,
    address next,
    uint limit,
    uint[2] exchangeRate,
    uint sourceBalance,
    uint targetBalance,
    uint sourceLockedBalance,
    uint targetLockedBalance
  ) {
    var _offer              = _getOffer(source, target);
    exists                  = _offer.exists;
    prev                    = _offer.prev;
    next                    = _offer.next;
    limit                   = _offer.limit;
    exchangeRate            = _offer.exchangeRate;
    sourceBalance           = _offer.sourceBalance;
    targetBalance           = _offer.targetBalance;
    sourceLockedBalance     = _offer.sourceLockedBalance;
    targetLockedBalance     = _offer.targetLockedBalance;
  }

  /**
   * @notice Creates a new Offer in the source user's OfferMap
   * @param source Address of source user
   * @param target Address of counterparty user
   * @return bool
   */
  function createOffer(
    address source,
    address target,
    uint limit,
    uint[2] exchangeRate
  ) public returns (bool) {
    if (offerExists(source, target)) { throw; }
    // TODO: replace this with a call that returns a pointer to storage
    // TODO: update this for adding to linked list
    var _offer = Offer(true, 0x0, 0x0, target, true, limit, exchangeRate, 0, 0, 0, 0);
    Graph.users[source].offerMap.offers[target] = _offer;
    return true;
  }

  /**
   * @notice Swaps two Offers' positions within a OfferMap
   * @param source Address of source user
   * @param targetOne Address of first Offer
   * @param targetTwo Address of second Offer
   * @return bool
   */
  function swapOffers(
    address source,
    address targetOne,
    address targetTwo
  ) public returns (bool) {}

  //////////////////////////////////////////////////////////////////////
  //////////////////////////////////////////////////////////////////////
  //////////////////////////////////////////////////////////////////////
  // public iterators
    // TODO: add remaining linked list iterators
  function iter_getOfferMapSize(
    address source,
    address target
  ) public constant returns (uint size) {}
  function iter_getFirstOffer(
    address source,
    address target
  ) public constant returns (
    uint limit,
    uint[2] exchangeRate,
    uint sourceBalance,
    uint targetBalance,
    uint lockedSourceBalance,
    uint lockedTargetBalance
  ) {}
  function iter_getPrevOffer(
    address source,
    address target
  ) public constant returns (
    uint limit,
    uint[2] exchangeRate,
    uint sourceBalance,
    uint targetBalance,
    uint lockedSourceBalance,
    uint lockedTargetBalance
  ) {}
  function iter_getNextOffer(
    address source,
    address target
  ) public constant returns (
    uint limit,
    uint[2] exchangeRate,
    uint sourceBalance,
    uint targetBalance,
    uint lockedSourceBalance,
    uint lockedTargetBalance
  ) {}

  /********************************************************//**
   * @struct User
   * @notice A Whuffie-holding account
   ***********************************************************/
  struct User {
    bool      exists;       /**< whether or not the User exists */
    address   sourceAddr;   /**< the User's address */
    string    metadata;     /**< metadata regarding the User's last transaction */
    OfferMap  offerMap;    /**< a collection of the User's open offers */
  }

  // internal helpers
  // TODO: audit these functions for redundancy, performance and "throw"-related errors
  /**
   * @notice Internal method for fetching a OfferMap struct from storage
   * @param source Address of source user for all offers in OfferMap
   * @return OfferMap instance
   */
  function _getOfferMap(
    address source
  ) internal constant returns (OfferMap) {
    return Graph.users[source].offerMap;
  }

  /**
   * @notice Fetches the latest IPFS hash for a user
   * @param source User's address
   * @return metadata IPFS hash of user's latest transaction
   */
  function getMetadata(
    address source
  ) public constant returns (string metadata) {
    return _getUser(source).metadata;
  }

  /**
   * @notice Sets latest IPFS hash for a user
   * @param source User's address
   * @param metadata IPFS hash of user's latest transaction
   * @return bool
   */
  function setMetadata(
    address source,
    string metadata
  ) public returns (bool) {
    if (userExists(source) == false) { throw; }
    var user = _getUser(source);
    user.metadata = metadata;
    return true;
  }

  /********************************************************//**
   * @struct UserMap
   * @notice A doubly-linked list containing all Whuffie Users and Offers
   ***********************************************************/
  struct UserMap {
    uint    size;         /**< length of the linked-list */
    address head;         /**< pointer to the first User of linked-list */
    address tail;         /**< pointer to the last User of linked-list */
    mapping (
      address => User     /**< hashmap of Users by their address */
    ) users;
  }

  /**
   * @notice Internal method for fetching a User struct from storage
   * @param source Address of desired user
   * @return User instance
   */
  // TODO: audit and test these functions for redundancy, performance and "throw"-related errors
  function _getUser(
    address source
  ) internal constant returns (User) {
    return Graph.users[source];
  }

  /**
   * @notice Returns User struct members for a given address
   * @dev Must return individual members (since solidity doesn't allow struct
   *  return values within the EVM)
   * @param source Address of the user
   * @return exists
   * @return metadata IPFS hash of the user's last transaction
   */
  function getUser(
    address source
  ) public constant returns (
    bool exists,
    string metadata
  ) {
    var _user = _getUser(source);
    exists = _user.exists;
    metadata = _user.metadata;
  }

  /**
   * @notice Determines if the User exists
   * @param source User's address
   * @return bool
   */
  function userExists(
    address source
  ) public constant returns (bool) {
    return _getUser(source).exists;
  }

  /**
   * @notice Creates a new User in Graph
   * @param source User's address
   * @param metadata IPFS hash of the user creation transaction
   */
  function createUser(
    address source,
    string metadata
  ) public isAPI returns (bool) {
    if (userExists(source)) { return false; }
    // TODO: replace this with a call that returns a pointer to storage
    Graph.users[source] = User(true, source, metadata, OfferMap(0, 0x0, 0x0));
    return true;
  }

  //////////////////////////////////////////////////////////////////////
  //////////////////////////////////////////////////////////////////////
  //////////////////////////////////////////////////////////////////////
  // public iterators
    // TODO: add remaining linked list iterators
  function iter_getUserMapSize(
    address source,
    address target
  ) public constant returns (uint size) {}
  function iter_getFirstUser(
    address source,
    address target
  ) public constant returns (
    uint limit,
    uint[2] exchangeRate,
    uint sourceBalance,
    uint targetBalance,
    uint lockedSourceBalance,
    uint lockedTargetBalance
  ) {}
  function iter_getPrevUser(
    address source,
    address target
  ) public constant returns (
    uint limit,
    uint[2] exchangeRate,
    uint sourceBalance,
    uint targetBalance,
    uint lockedSourceBalance,
    uint lockedTargetBalance
  ) {}
  function iter_getNextUser(
    address source,
    address target
  ) public constant returns (
    uint limit,
    uint[2] exchangeRate,
    uint sourceBalance,
    uint targetBalance,
    uint lockedSourceBalance,
    uint lockedTargetBalance
  ) {}

  // fallback function
  function() { throw; }
}
