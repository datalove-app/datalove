import "lib/assertive.sol";
import "lib/activable.sol";
import "lib/restrictedAPI.sol";

import "lib/whuffie.types.sol";
import "lib/whuffie.accounts.sol";
// import "lib/whuffie.credits.sol";
// import "lib/whuffie.offers.sol";

// TODO: should be stored in some kind of NameReg
// TODO: audit and test all functions for redundancy, performance and "throw"-related errors

/**
 * @title WhuffieDB
 * @author Sunny Gonnabathula | @sunny-g | sunny.gonna@gmail.com
 * @notice Implements public getters and API-restricted setters and iterators
 *  for the Whuffie Graph
 * @dev This contract will maintain the base-level storage of all Accounts and Offers,
 *  and will only be mutated by selected API contracts.
 */
contract WhuffieDB is Assertive, Activable, RestrictedAPI {
  using Accounts for Types.AccountMap;
  // using Account for Types.Account;
  // using Credits for Types.CreditMap;
  // using Credit for Types.Credit;
  // using Offers for Types.OfferMap;
  // using Offer for Types.Offer;

  Types.AccountMap public Graph;
  uint constant MAXUINT = 2**256 - 1;

  function WhuffieDB () {
    Graph = Types.AccountMap(0, 0x0, 0x0);

    // for testing purposes
    activate();
    addAPI(0xdedb49385ad5b94a16f236a6890cf9e0b1e30392, "test");
  }

  /********************************************************//**
   * @notice AccountMap - A linked hashmap containing all Whuffie Accounts and their Offers
   ***********************************************************/
  /**
   * @notice Returns Account struct members for a given address
   * @dev Must return individual members (since solidity doesn't allow struct
   *  return values within the EVM)
   * @param source Address of the account
   */
  function getAccount(
    address source
  ) public constant returns (
    bytes32   metadata,
    address   owner,
    bytes12   creditSymbol,
    bytes32   creditName,
    uint      totalSupply,
    uint      sourceBalance,
    uint      sourceFrozenBalance,
    uint8     decimals,
    bool      exists,
    address   prevAddr,
    address   nextAddr
  ) {
    Types.Account storage account = Graph.getAccount(source);
    metadata              = account.metadata;
    creditSymbol          = account.creditSymbol;
    creditName            = account.creditName;
    totalSupply           = account.totalSupply;
    sourceBalance         = account.sourceBalance;
    sourceFrozenBalance   = account.sourceFrozenBalance;
    decimals              = account.decimals;
    exists                = account.exists;
    owner                 = account.owner;
    prevAddr              = account.prevAddr;
    nextAddr              = account.nextAddr;
  }

  /**
   * @notice Determines if an Account of this address has ever been created
   * @param source Account's address
   * @return bool
   */
  function accountExists(
    address source
  ) public constant returns (bool exists) {
    return Graph.getAccount(source).exists;
  }

  /**
   * @notice Creates a new Account in the Graph
   * @param source Account's address
   * @param metadata IPFS hash of the account creation transaction
   */
  function createAccount(
    address source,
    address owner,
    bytes12 creditSymbol,
    bytes32 creditName,
    uint8   decimals,
    uint    initialTotalSupply,
    uint    initialSourceBalance,
    bytes32 metadata
  ) public onlyAPI onlyActivated returns (bool success) {
    assert(!accountExists(source));
    assert(Graph.size != MAXUINT);

    assert(Graph.createAccount(
      source, owner, creditSymbol, creditName,
      decimals, initialTotalSupply, initialSourceBalance, metadata
    ));
    return true;
  }

  /********************************************************//**
   * @notice Account - A Whuffie-holding account
   ***********************************************************/
  /**
   * @notice Fetches the latest metadata for a account
   * @param source Account's address
   * @return metadata Metadata pertaining to the account's latest transaction
   */
  function getMetadata(
    address source
  ) public constant returns (bytes32 metadata) {
    return Graph.getAccount(source).metadata;
  }

  /**
   * @notice Sets latest metadata for a account
   * @param source Account's address
   * @param metadata Metadata pertaining to the account's latest transaction
   * @return bool
   */
  function setMetadata(
    address source,
    bytes32 metadata
  ) public onlyAPI onlyActivated returns (bool success) {
    Types.Account storage account = Graph.getAccount(source);
    assert(account.exists);

    account.metadata = metadata;
    return true;
  }

  // internal helpers
  function _setAccountPrevAddr(
    address source,
    address prevAddr
  ) internal returns (bool success) {
    Types.Account storage account = Graph.getAccount(source);
    assert(account.exists);

    account.prevAddr = prevAddr;
    return true;
  }

  function _setAccountNextAddr(
    address source,
    address nextAddr
  ) internal returns (bool success) {
    Types.Account storage account = Graph.getAccount(source);
    assert(account.exists);

    account.nextAddr = nextAddr;
    return true;
  }

  /********************************************************//**
   * @notice CreditMap - A linked hashmap containing all of an Account's issued credits
   * @dev O(1) get, add, remove, swap
   ***********************************************************/
  // struct CreditMap {
  //   uint    size;           /**< length of the linked-list */
  //   address firstAddr;      /**< source address of first Credit of linked-list */
  //   address lastAddr;       /**< source address of last Credit of linked-list */
  //   mapping (
  //     bytes12 => Credit     /**< hashmap of Credits by symbol */
  //   ) credits;
  // }

  // /********************************************************//**
  // * @struct Credit
  // * @notice
  // ***********************************************************/
  // struct Credit {

  // }

  // /**
  // * @notice Fetches the name of the Account's credit
  // * @param source Account's address
  // * @return name Name of the Account's credit
  // */
  // function getCreditName(
  //   address source
  // ) public constant returns (bytes32 creditName) {
  //   return getAccount(source).creditName;
  // }

  // /**
  // * @notice Sets the name of the Account's credit
  // * @param source Account's address
  // * @param name New name of the Account's credit
  // * @return bool
  // */
  // function setCreditName(
  //   address source,
  //   bytes32 creditName
  // ) public onlyAPI onlyActivated returns (bool success) {
  //   assert(accountExists(source));

  //   Graph.accounts[source].creditName = creditName;
  //   return true;
  // }

  // /**
  // * @notice Fetches the symbol of the Account's credit
  // * @param source Account's address
  // * @return symbol Symbol of the Account's credit
  // */
  // function getCreditSymbol(
  //   address source
  // ) public constant returns (bytes32 creditSymbol) {
  //   return getAccount(source).creditSymbol;
  // }

  // /**
  // * @notice Sets symbol for the Account's credit
  // * @param source Account's address
  // * @param symbol New symbol for the Account's credit
  // * @return bool
  // */
  // function setCreditSymbol(
  //   address source,
  //   bytes32 symbol
  // ) public onlyAPI onlyActivated returns (bool success) {
  //   assert(accountExists(source));

  //   Graph.accounts[source].creditSymbol = creditSymbol;
  //   return true;
  // }

  // /**
  // * @notice Fetches the number of decimal places for the Account's credits
  // * @param source Account's address
  // * @return decimals Number of decimal places
  // */
  // function getDecimals(
  //   address source
  // ) public constant returns (uint decimals) {
  //   return getAccount(source).decimals;
  // }

  // /**
  // * @notice Sets the decimal places for the Account's credit
  // * @param source Account's address
  // * @param decimals New decimal places for the Account's credit to have
  // * @return bool
  // */
  // function setDecimals(
  //   address source,
  //   uint decimals
  // ) public onlyAPI onlyActivated returns (bool success) {
  //   assert(accountExists(source));

  //   Graph.accounts[source].decimals = decimals;
  //   return true;
  // }

  // /********************************************************//**
  // * @struct OfferMap
  // * @notice A linked hashmap containing all of an Account's open Offers
  // * @dev O(1) get, add, remove, swap
  // ***********************************************************/
  // struct OfferMap {
  //   uint    size;           /**< length of the linked-list */
  //   address firstAddr;      /**< source address of first Offer of linked-list */
  //   address lastAddr;       /**< source address of last Offer of linked-list */
  //   mapping (
  //     address => Offer      /**< hashmap of Offers by target address */
  //   ) offers;
  // }

  // /**
  // * @notice Internal method for fetching an individual Offer
  // * @param source Address of source account
  // * @param target Address of counterparty account
  // * @return Offer instance
  // */
  // function _getOffer(
  //   address source,
  //   address target
  // ) internal constant returns (Offer) {
  //   return Graph.accounts[source].offerMap.offers[target];
  // }

  // /**
  // * @notice Returns Offer struct members
  // * @dev Must return individual members (since solidity doesn't allow struct
  // *  return values within the EVM)
  // * @param source Address of source account
  // * @param target Address of counterparty account
  // * @return exists
  // * @return prev
  // * @return next
  // * @return limit
  // * @return exchangeRate
  // * @return sourceBalance
  // * @return targetBalance
  // * @return frozenSourceBalance
  // * @return frozenTargetBalance
  // */
  // function getOffer(
  //   address source,
  //   address target
  // ) public constant returns (
  //   bool exists,
  //   address prevAddr,
  //   address nextAddr,
  //   bool active,
  //   uint limit,
  //   uint[2] exchangeRate,
  //   uint sourceBalance,
  //   uint targetBalance,
  //   uint sourceFrozenBalance,
  //   uint targetFrozenBalance
  // ) {
  //   var _offer              = _getOffer(source, target);
  //   exists                  = _offer.exists;
  //   prevAddr                = _offer.prevAddr;
  //   nextAddr                = _offer.nextAddr;
  //   active                  = _offer.active;
  //   limit                   = _offer.limit;
  //   exchangeRate            = _offer.exchangeRate;
  //   sourceBalance           = _offer.sourceBalance;
  //   targetBalance           = _offer.targetBalance;
  //   sourceFrozenBalance     = _offer.sourceFrozenBalance;
  //   targetFrozenBalance     = _offer.targetFrozenBalance;
  // }

  // function _createOffer(
  //   address source,
  //   address target,
  //   Offer offer
  // ) internal returns (bool success) {
  //   Graph.accounts[source].offerMap.offers[target] = offer;
  //   return true;
  // }

  // /**
  // * @notice Creates a new Offer in the source account's OfferMap
  // * @param source Address of source account
  // * @param target Address of counterparty account
  // * @return bool
  // */
  // function createOffer(
  //   address source,
  //   address target,
  //   uint limit,
  //   uint[2] exchangeRate
  // ) public onlyAPI onlyActivated returns (bool success) {
  //   assert(offerExists(source, target));
  //   var size = getOfferMapSize(source);
  //   assert(size == MAX_UINT);

  //   var _newOffer = Offer(true, 0x0, 0x0, target, true, limit, exchangeRate, 0, 0, 0, 0);

  //   if (size == 0) {
  //     Graph.accounts[source].offerMap.firstAddr = target;
  //   } else {
  //     address oldTailAddr = Graph.accounts[source].offerMap.lastAddr;
  //     _setOfferNextAddr(source, oldTailAddr, target);
  //     _newOffer.prevAddr = oldTailAddr;
  //   }

  //   Graph.accounts[source].offerMap.lastAddr = target;
  //   Graph.accounts[source].offerMap.size = size + 1;
  //   Graph.accounts[source].offerMap.offers[target] = _newOffer;
  //   return true;
  // }

  // // TODO: audit these functions for redundancy, performance and "throw"-related errors
  // /**
  // * @notice Fetches the length of the OfferMap
  // * @param source The source account's OfferMap
  // * @return uint Size of the OfferMap
  // */
  // function getOfferMapSize(
  //   address source
  // ) public constant returns (uint size) {
  //   return Graph.accounts[source].offerMap.size;
  // }

  // /**
  // * @notice Swaps two Offers' positions within an OfferMap
  // * @param source Address of source account
  // * @param targetOne Address of first Offer
  // * @param targetTwo Address of second Offer
  // * @return bool
  // */
  // function swapOffers(
  //   address source,
  //   address targetOne,
  //   address targetTwo
  // ) public onlyAPI onlyActivated returns (bool success) {
  //   assert(offerExists(source, targetOne));
  //   assert(offerExists(source, targetTwo));
  // }

  // /********************************************************//**
  // * @struct Offer
  // * @notice An offer to exchange the source's credits for the target's
  // ***********************************************************/
  // struct Offer {
  //   bool    exists;               /**< whether or not an Offer has been created */
  //   address prevAddr;             /**< target address of previous Offer in linked-list */
  //   address nextAddr;             /**< target address of next Offer's in linked-list */

  //   address targetAddr;           /**< address of Offer target */
  //   bool    active;               /**< whether or not the Offer can be used in transactions */
  //   uint    limit;                /**< maximum amount of target credit to hold */
  //   // TODO: fix this to be more ERC20-compliant
  //   uint[2] exchangeRate;         /**< exchange rate between target's and source's credit */
  //   uint    sourceBalance;        /**< balance of source's credit */
  //   uint    targetBalance;        /**< balance of target's credit */
  //   uint    sourceFrozenBalance;  /**< immovable balance of source's credit */
  //   uint    targetFrozenBalance;  /**< immovable balance of target's credit */
  // }

  // /**
  // * @notice Determines if an Offer has ever been created
  // * @param source Address of source account
  // * @param target Address of counterparty account
  // * @return bool
  // */
  // function offerExists(
  //   address source,
  //   address target
  // ) public constant returns (bool exists) {
  //   return _getOffer(source, target).exists;
  // }

  // /**
  // * @notice Determines if a Offer is alive and usable for trades
  // * @param source Address of source account
  // * @param target Address of counterparty account
  // * @return bool
  // */
  // function offerIsActive(
  //   address source,
  //   address target
  // ) public constant returns (bool active) {
  //   return _getOffer(source, target).active;
  // }

  // /**
  // * @param source Address of the offer owner
  // * @param target Address of the offer's counterparty
  // * @param activeStatus The new activity status of the offer
  // * @return success
  // */
  // function setOfferActiveStatus(
  //   address source,
  //   address target,
  //   bool activeStatus
  // ) public onlyAPI onlyActivated returns (bool success) {
  //   assert(offerExists(source, target));

  //   Graph.accounts[source].offerMap.offers[target].active = activeStatus;
  //   return true;
  // }

  // /**
  // * @param source Address of the offer owner
  // * @param target Address of the offer's counterparty
  // * @return limit Maximum amount of target credit to hold
  // */
  // function getOfferLimit(
  //   address source,
  //   address target
  // ) public constant returns (uint limit) {
  //   return _getOffer(source, target).limit;
  // }

  // /**
  // * @param source Address of the offer owner
  // * @param target Address of the offer's counterparty
  // * @param limit The new limit of target credit to hold
  // * @return success
  // */
  // function setOfferLimit(
  //   address source,
  //   address target,
  //   uint limit
  // ) public onlyAPI onlyActivated returns (bool success) {
  //   assert(offerExists(source, target));

  //   Graph.accounts[source].offerMap.offers[target].limit = limit;
  //   return true;
  // }

  // /**
  // * @param source Address of the offer owner
  // * @param target Address of the offer's counterparty
  // * @return sourceBalance Amount of source Account's credit held within this Offer
  // */
  // function getOfferSourceBalance(
  //   address source,
  //   address target
  // ) public constant returns (uint sourceBalancec) {
  //   return _getOffer(source, target).sourceBalance;
  // }

  // /**
  // * @param source Address of the offer owner
  // * @param target Address of the offer's counterparty
  // * @param sourceBalance New amount of source Account's credit to be held within this Offer
  // * @return success
  // */
  // function setOfferSourceBalance(
  //   address source,
  //   address target,
  //   uint sourceBalance
  // ) public onlyAPI onlyActivated returns (bool success) {
  //   assert(offerExists(source, target));

  //   Graph.accounts[source].offerMap.offers[target].sourceBalance = sourceBalance;
  //   return true;
  // }

  // /**
  // * @param source Address of the offer owner
  // * @param target Address of the offer's counterparty
  // * @return sourceBalance Amount of target Account's credit held within this Offer
  // */
  // function getOfferTargetBalance(
  //   address source,
  //   address target
  // ) public constant returns (uint targetBalance) {
  //   return _getOffer(source, target).targetBalance;
  // }

  // /**
  // * @param source Address of the offer owner
  // * @param target Address of the offer's counterparty
  // * @param targetBalance New amount of target Account's credit to be held within this Offer
  // * @return success
  // */
  // function setOfferTargetBalance(
  //   address source,
  //   address target,
  //   uint targetBalance
  // ) public onlyAPI onlyActivated returns (bool success) {
  //   assert(offerExists(source, target));

  //   Graph.accounts[source].offerMap.offers[target].targetBalance = targetBalance;
  //   return true;
  // }

  // function getOfferExchangeRate(
  //   address source,
  //   address target
  // ) public constant returns (uint[2] exchangeRate) {
  //   return _getOffer(source, target).exchangeRate;
  // }

  // function setOfferExchangeRate(
  //   address source,
  //   address target,
  //   uint[2] exchangeRate
  // ) public onlyAPI onlyActivated returns (bool success) {
  //   assert(offerExists(source, target));

  //   Graph.accounts[source].offerMap.offers[target].exchangeRate = exchangeRate;
  //   return true;
  // }

  // /**
  // * @param source Address of the offer owner
  // * @param target Address of the offer's counterparty
  // * @return sourceFrozenBalance Amount of source Account's credit frozen within this Offer
  // */
  // function getOfferFrozenSourceBalance(
  //   address source,
  //   address target
  // ) public constant returns (uint sourceFrozenBalance) {
  //   return _getOffer(source, target).sourceFrozenBalance;
  // }

  // /**
  // * @param source Address of the offer owner
  // * @param target Address of the offer's counterparty
  // * @param sourceFrozenBalance New amount of source Account's credit to be frozen within this Offer
  // * @return success
  // */
  // function setOfferFrozenSourceBalance(
  //   address source,
  //   address target,
  //   uint sourceFrozenBalance
  // ) public onlyAPI onlyActivated returns (bool success) {
  //   assert(offerExists(source, target));

  //   Graph.accounts[source].offerMap.offers[target].sourceFrozenBalance = sourceFrozenBalance;
  //   return true;
  // }

  // /**
  // * @param source Address of the offer owner
  // * @param target Address of the offer's counterparty
  // * @return targetFrozenBalance Amount of target Account's credit frozen within this Offer
  // */
  // function getOfferFrozenTargetBalance(
  //   address source,
  //   address target
  // ) public constant returns (uint targetFrozenBalance) {
  //   return _getOffer(source, target).targetFrozenBalance;
  // }

  // /**
  // * @param source Address of the offer owner
  // * @param target Address of the offer's counterparty
  // * @param targetFrozenBalance New amount of target Account's credit to be frozen within this Offer
  // * @return success
  // */
  // function setOfferFrozenTargetBalance(
  //   address source,
  //   address target,
  //   uint targetFrozenBalance
  // ) public onlyAPI onlyActivated returns (bool success) {
  //   assert(offerExists(source, target));

  //   Graph.accounts[source].offerMap.offers[target].targetFrozenBalance = targetFrozenBalance;
  //   return true;
  // }

  // // internal helpers
  // function _setOfferPrevAddr(
  //   address source,
  //   address target,
  //   address prevAddr
  // ) internal returns (bool success) {
  //   assert(offerExists(source, target));

  //   Graph.accounts[source].offerMap.offers[target].prevAddr = prevAddr;
  //   return true;
  // }

  // function _setOfferNextAddr(
  //   address source,
  //   address target,
  //   address nextAddr
  // ) internal returns (bool success) {
  //   assert(offerExists(source, target));

  //   Graph.accounts[source].offerMap.offers[target].nextAddr = nextAddr;
  //   return true;
  // }

  // fallback function
  function() { throw; }
}
