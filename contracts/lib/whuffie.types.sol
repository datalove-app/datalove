/**
 * @notice Schemas/Models used in the WhuffieDB
 */
library Types {
  /********************************************************//**
   * @struct AccountMap
   * @dev A linked hashmap containing all Whuffie Accounts and their Offers
   ***********************************************************/
  struct AccountMap {
    uint      size;         /**< length of the linked-list */
    address   firstAddr;    /**< source address of first Account of linked-list */
    address   lastAddr;     /**< source address of last Account of linked-list */
    mapping (
      address => Account    /**< hashmap of Accounts by their address */
    ) accounts;
  }

  /********************************************************//**
   * @struct Account
   * @notice A Whuffie-holding account
   ***********************************************************/
  struct Account {
    bytes32   metadata;             /**< metadata regarding the Account's last transaction */
    address   sourceAddr;           /**< the Account's address */
    bytes12   creditSymbol;         /**< symbol of this Account's credit, 4-12 alphanumeric characters */
    bytes32   creditName;           /**< name of this credit */
    uint      totalSupply;          /**< total supply of the credit */
    uint      sourceBalance;        /**< balance of source's credit */
    uint      sourceFrozenBalance;  /**< immovable balance of source's credit */
    uint8     decimals;             /**< number of decimal places to show */

    bool      exists;               /**< whether or not the Account exists */
    address   owner;                /**< address of owner Account, 0x0 if not owned */
    address   prevAddr;             /**< source address of previous Account in AccountMap */
    address   nextAddr;             /**< source address of next Account in AccountMap */

    CreditMap creditMap;            /**< collection of the Credits issued by the Account */
    OfferMap  offerMap;             /**< collection of the Offers created by the Account */
  }

  /********************************************************//**
   * @struct CreditMap
   * @notice A linked hashmap of an Account's issued credits
   * @dev O(1) get, add, remove, swap
   ***********************************************************/
  struct CreditMap {
    uint      size;         /**< length of the linked-list */
    bytes12   firstSymbol;
    bytes12   lastSymbol;
    mapping (
      bytes12 => Credit
    ) credits;              /**< hashmap of Credits by symbol */
  }

  /********************************************************//**
   * @struct Credit
   * @notice Accounts controlled by this
   ***********************************************************/
  struct Credit {
    address   issuerAddr;   /**< Account issuing this credit */
    bytes12   creditSymbol; /**< symbol of this credit, 4-12 alphanumeric characters */
    address   prevAddr;     /**< source address of previous Credit in CreditMap */
    address   nextAddr;     /**< source address of next Credit in CreditMap */
    bool      exists;       /**< whether or not the Credit has been initialized */
  }

  /********************************************************//**
   * @struct OfferMap
   * @notice A linked hashmap containing all of an Account's open Offers
   * @dev O(1) get, add, remove, swap
   ***********************************************************/
  struct OfferMap {
    uint      size;         /**< length of the linked-list */
    address   firstAddr;    /**< source address of first Offer of linked-list */
    address   lastAddr;     /**< source address of last Offer of linked-list */
    mapping (
      address => Offer      /**< hashmap of Offers by target address */
    ) offers;
  }

  /********************************************************//**
   * @struct Offer
   * @notice An offer to exchange the source's credits for the target's
   ***********************************************************/
  struct Offer {
    address   targetAddr;           /**< address of Offer target */
    bytes12   targetCreditSymbol;   /**< code representing the target's credit */
    uint      limit;                /**< maximum amount of target credit to hold */
    // TODO: fix this to be more a ERC20-compliant exchange rate
    uint[2]   exchangeRate;         /**< exchange rate between target's and source's credit */
    uint      targetBalance;        /**< balance of target's credit */
    uint      targetFrozenBalance;  /**< immovable balance of target's credit */

    bool      active;               /**< whether or not this Offer can be used in transactions */
    bool      exists;               /**< whether or not an Offer has been initialized */
    address   prevAddr;             /**< target address of previous Offer in OfferMap */
    address   nextAddr;             /**< target address of next Offer in OfferMap */
  }
}
