import "lib/whuffie.types.sol";
import "lib/whuffie.accounts.sol";

/**
 */
library Offers {
  using Accounts for Types.AccountMap;
  using Account for Types.Account;

  // /**
  // * @notice Internal method for fetching an individual Offer
  // * @param source Address of source account
  // * @param target Address of counterparty account
  // * @return Offer instance
  // */
  // function _getOffer(
  //   OfferMap storage offerMap,
  //   address source
  // ) internal constant returns (Offer) {
  //   return offerMap.offers[target];
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
  // ) internal returns (bool success) {
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
  // ) internal returns (bool success) {
  //   assert(offerExists(source, targetOne));
  //   assert(offerExists(source, targetTwo));
  // }
}

library Offer {
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
  // ) internal returns (bool success) {
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
  // ) internal returns (bool success) {
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
  // ) internal returns (bool success) {
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
  // ) internal returns (bool success) {
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
  // ) internal returns (bool success) {
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
  // ) internal returns (bool success) {
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
  // ) internal returns (bool success) {
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
}