/**
 * Method interfaces
 */

interface IEntryWithHash<TEntry> extends IHash, IEntry<TEntry>{}

interface IUpdateAgentOptionsIdentity {
  Identity: string,
}

interface IUpdateAgentOptionsRevocation {
  Revocation: string,
}

type IUpdateAgentOptions =
  | IUpdateAgentOptionsIdentity
  | IUpdateAgentOptionsRevocation
  | (IUpdateAgentOptionsIdentity & IUpdateAgentOptionsRevocation);

interface IGetOptions {
  StatusMask?: HCStatus,
  GetMask?: HCGetMask,
  Local?: boolean,
  Bundle?: boolean,
}

interface IGetLinksOptions {
  StatusMask: HCStatus,
}

interface IGetLinksOptionsLoad {
  Load: true,
  StatusMask?: HCStatus,
}

interface IQueryOptionsConstrain {
  EntryTypes: string[],
  Contains?: string,
  Equals?: string,
  Matches?: RegExp,
  Count?: Integer,
  Page?: Integer,
  // AfterHash: Hash,
  // BeforeHash: Hash,
  // Negate: boolean
}

interface IQueryOptionsOrder {
  Ascending?: boolean,
  // Criteria: any,
}

interface IQueryOptionsBase {
  Constrain: IQueryOptionsConstrain,
  Order?: IQueryOptionsOrder,
  Bundle?: boolean,
}

interface IQueryOptionsHashes extends IQueryOptionsBase {
  Return: {
    Hashes: true,
  },
}

interface IQueryOptionsHeaders extends IQueryOptionsBase {
  Return: {
    Headers: true,
  },
}

interface IQueryOptionsEntries extends IQueryOptionsBase {
  Return: {
    Entries: true,
  },
}

type IQueryResponseHash = Hash;
type IQueryResponseHeader = IHeader;
type IQueryResponseEntry<TEntry> = TEntry;
interface IQueryResponseHashAndEntry<TEntry> {
  Entry: TEntry,
  Hash: Hash,
}

interface ISendOptions {
  Function: string,
  ID: string,
}

/**
 * Sends output to the debugging log.
 * @param value
 */
declare function debug(value: any): void;

/**
 * Returns an application property, which are defined by the app developer as properties in the DNA file (e.g. Name, Language, Description, Author, etc.).
 * @param {string} propertyName
 * @returns {TReturn} TReturn extends string | Either<string> = Either<string>
 */
declare function property<TReturn extends string | Either<string> = Either<string>>(propertyName: string): TReturn;

/**
 * Calls a bridged function from another app. Just like in `send`, the `arguments` parameter is a string or an object/hash depending on the CallingType that was specified in the function's definition. Returns the value that's returned by the given function on the other side of the bridge.
 *
 * NOTE: the application being called must have explicitly been bridged.
 * @param {Hash} appDNAHash
 * @param {string} zomeName
 * @param {string} functionName
 * @param {string | TArgs} arguments
 * @returns {TReturn} TReturn extends any | Either<any> = Either<any>
 */
declare function bridge<TArgs, TReturn extends any | Either<any> = Either<any>>(appDNAHash: Hash, zomeName: string, functionName: string, arguments: string | TArgs): TReturn; // FIXME: could return error?

/**
 * Calls and exposed function from another zome. Just like in `send`, the `arguments` parameter is a string or an object/hash depending on the CallingType that was specified in the function's definition. Returns the value that's returned by the given function.
 * @param {string} zomeName
 * @param {string} functionName
 * @param {TArgs} arguments
 * @returns {TReturn} TReturn extends any | Either<any> = Either<any>
 */
declare function call<TArgs extends (string | object), TReturn extends any | Either<any> = Either<any>>(zomeName: string, functionName: string, arguments: TArgs): TReturn;

/**
 * Sends a message to a node, using the `App.Key.Hash` of that node, its permanent address in the DHT. The return value of this function will be whatever is returned by the `receive` callback on the receiving node. Alternatively, you can indicate that this call should be made asynchronously, and specify the callback function using the `ISendOptions` properties.
 * @param {Hash} nodeHash
 * @param {TMessage} message
 * @param {ISendOptions} options
 * @returns {TReturn} TReturn extends any | Either<any> = Either<any>
 */
declare function send<TMessage extends object, TReturn extends any | Either<any> = Either<any>>(nodeHash: Hash, message: TMessage, options?: ISendOptions): TReturn;

/**
 * Allows the app to examine which bridges have been put in place.
 * @returns {IBridge[]}
 */
declare function getBridges(): IBridge[];

/**
 * Retrieves a list of links tagged as `tag` on `base` from the DHT. If `tag` is an empty string, it will return all the links on the `base` and the list will also include the `Tag` property on entries.
 *
 * With `IGetLinksOptions` (the default), this returns an IHash[]
 * With `IGetLinksOptionsLoad`, this returns an IEntryWithHash<TEntry>[]
 * Use `options.StatusMask` with an `HCStatus` a sum of enum values to return only links with a certain status (default is `HC.Status.Live`).
 * @param {Hash} base
 * @param {string} tag
 * @param {IGetLinksOptions} options
 * @returns {TReturn} TReturn extends IHash[] | Either<IHash[]> = Either<IHash[]>
 */
declare function getLinks<TReturn extends IHash[] | Either<IHash[]> = Either<IHash[]>>(base: Hash, tag: string, options: IGetLinksOptions): TReturn;

/**
 * @see getLinks
 * @param {Hash} base
 * @param {string} tag
 * @param {IGetLinksOptionsLoad} options
 * @returns {TReturn} TReturn extends IEntryWithHash<TEntry>[] | Either<IEntryWithHash<TEntry>[]> = Either<IEntryWithHash<TEntry>[]>
 */
declare function getLinks<TEntry, TReturn extends IEntryWithHash<TEntry>[] | Either<IEntryWithHash<TEntry>[]> = Either<IEntryWithHash<TEntry>[]>>(base: Hash, tag: string, options: IGetLinksOptionsLoad): TReturn;

/**
 * Retrieves an entry from the local chain or the DHT.
 *
 * If `options.StatusMask` is present, it determines which entries to return, depending on their status.
 * If `options.GetMask` is present, this option allows you to specify what information you want about the entry.
 * If `options.Local` is `true`, this will retrieve specific entries from your local chain only.
 * If `options.Bundle` is `true`, this will retrieve entries from the currently started bundle *only* - if no bundle has been started, this returns an error. FIXME: return or throw an error?
 *
 * @param {Hash} hash
 * @param {IGetOptions} options
 * @returns {HashNotFound | TReturn}
 */
declare function get<TReturn>(hash: Hash, options?: IGetOptions): HashNotFound | TReturn;

// /**
//  *
//  * @param {IQueryOptionsHashes} options
//  * @returns {IQueryResponseHash[] | IError}
//  */
// declare function query(options?: IQueryOptionsHashes): IQueryResponseHash[] | IError;
// declare function query(options?: IQueryOptionsHeaders): IQueryResponseHeader[] | IError;
// declare function query(options?: IQueryOptionsHashes & IQueryOptionsHeaders): IError; // FIXME: what does it look like to return hashes and headers?
// declare function query<TEntry>(options?: IQueryOptionsEntries): IQueryResponseEntry<TEntry>[] | IError;
// declare function query<TEntry>(options?: IQueryOptionsEntries & IQueryOptionsHashes): IQueryResponseHashAndEntry<TEntry>[] | IError;
// declare function query<TEntry>(options?: IQueryOptionsEntries & IQueryOptionsHeaders): IError
// declare function query<TEntry>(options?: IQueryOptionsEntries & IQueryOptionsHashes & IQueryOptionsHeaders): IError

/**
 *
 * @param {string} entryType
 * @param {string | {Links: ILink[]}} entryData
 * @returns {TReturn} TReturn extends Hash | Either<Hash> = Either<Hash>
 */
declare function commit<TReturn extends Hash | Either<Hash> = Either<Hash>>(entryType: string, entryData: string | { Links: ILink[] }): TReturn;

/**
 * @see commit
 * @param {string} entryType
 * @param {TEntry} entryData
 * @returns {TReturn} TReturn extends Hash | Either<Hash> = Either<Hash>
 */
declare function commit<TEntry, TReturn extends Hash | Either<Hash> = Either<Hash>>(entryType: string, entryData: TEntry): TReturn;

/**
 *
 * @param {string} entryType
 * @param {string} entryData
 * @param {Hash} replaces
 * @returns {TReturn} TReturn extends Hash | Either<Hash> = Either<Hash>
 */
declare function update<TReturn extends Hash | Either<Hash> = Either<Hash>>(entryType: string, entryData: string, replaces: Hash): TReturn;

/**
 *
 * @param {string} entryType
 * @param {TEntry} entryData
 * @param {Hash} replaces
 * @returns {TReturn} TReturn extends Hash | Either<Hash> = Either<Hash>
 */
declare function update<TEntry extends object, TReturn extends Hash | Either<Hash> = Either<Hash>>(entryType: string, entryData: TEntry, replaces: Hash): TReturn;

/**
 *
 * @param {IUpdateAgentOptions} options
 * @returns {TReturn} TReturn extends Hash | Either<Hash> = Either<Hash>
 */
declare function updateAgent<TReturn extends Hash | Either<Hash> = Either<Hash>>(options: IUpdateAgentOptions): TReturn;

/**
 *
 * @param {Hash} entry
 * @param {string} message
 * @returns {Hash}
 */
declare function remove(entry: Hash, message?: string): Hash; // FIXME: message is optional? can return an Error?

/**
 *
 * @param {Integer} timeout
 * @param {TBundleStartParam} userParam
 */
declare function bundleStart<TBundleStartParam>(timeout: Integer, userParam: TBundleStartParam): void;

/**
 *
 * @param {boolean} commit
 */
declare function bundleClose(commit: boolean): void;
