// Type definitions for holochain 0.0.1
// Project: https://github.com/datalove-app/js-datalove

/* eslint-disable indent, no-restricted-globals, no-unused-vars, space-infix-ops */

declare type Bridge = IBridgeCaller | IBridgeCallee;
declare type DateTime = string; // looks like 2018-03-21T11:37:15Z
declare type Integer = number;
declare type Hash = string;
declare type HashNotFound = null;
declare type LinkTag = string;
declare type Sources = string[];
declare type Token = string;

/**
 * Enums
 */

declare enum HCStatus {
  Live = 1,
  Rejected = 2,
  Deleted = 4,
  Modified = 8,
  Any = 255,
}
declare enum HCGetMask {
  Default = 0,
  Entry = 1,
  EntryType = 2,
  Sources = 4,
  All = 255,
}
declare enum HCLinkAction {
  Add = '',
  Del = 'd',
}
declare enum HCBridge {
  Caller = 0,
  Callee = 1,
}
declare enum HCSysEntryType {
  DNA = '%dna',
  Agent = '%agent',
  Key = '%key',
  Headers = '%header',
  Del = '%del',
  Migrate = '%migrate',
}
declare enum HCPkgReqChainOpts {
  None = 0,
  Headers = 1,
  Entries = 2,
  Full = 3,
}
declare enum HCBundleCancelReason {
  UserCancel = 'userCancel',
  Timeout = 'timeout',
}
declare enum HCBundleCancelResponse {
  OK = '',
  Commit = 'commit',
}
declare enum HCMigrate {
  Close = 'close',
  Open = 'open',
}

/**
 * Interfaces
 */

interface IAppAgent {
  Hash: Hash,
  TopHash: Hash,
  String: string,
}

interface IAppDNA {
  Hash: Hash,
}

interface IAppKey {
  Hash: Hash,
}

interface IBridgeCaller {
  Side: HCBridge.Caller,
  CalleeApp: Hash,
  CalleeName: string,
}

interface IBridgeCallee {
  Side: HCBridge.Callee,
  Token: Token,
}

interface IEntry<TEntry> {
  EntryType: string,
  Entry: TEntry,
  Source: Hash,
}

interface IEntryWithHash<TEntry> extends IEntry<TEntry> {
  Hash: Hash,
}

interface IError {
  name: string, // FIXME: must prefixed by 'HolochainError'?
}

interface IHeader {
  EntryLink: Hash,
  Type: string,
  Time: DateTime,
}

interface ILink {
  Base: Hash,
  Link: Hash,
  Tag: LinkTag,
  LinkAction?: HCLinkAction.Del,
}

interface IPackage {
  Chain: {
    Emap?: { [hash: string]: Integer, },
    Entries?: object,
    Hashes?: { [hash: string]: Hash, },
    Headers?: IHeader[],
    Hmap?: { [hash: string]: Integer, },
    TypeTops?: { [hcSysEntry: string]: Integer, },
  },
}

/**
 * Method interfaces
 */

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
  Load?: false,
  StatusMask?: HCStatus,
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
 * Zome required callback interfaces
 */

interface IGenesis {
  (): void,
}
interface IValidateCommit<TEntry, TPackage extends IPackage> {
  (entryType: string, entry: TEntry, header: IHeader, pkg: null | TPackage, sources: Sources): boolean,
}
interface IValidatePut<TEntry, TPackage extends IPackage> {
  (entryType: string, entry: TEntry, header: IHeader, pkg: null | TPackage, sources: Sources): boolean,
}
interface IValidateMod<TEntry, TPackage extends IPackage> {
  (entryType: string, entry: TEntry, header: IHeader, replaces: Hash, pkg: null | TPackage, sources: Sources): boolean,
}
interface IValidateDel<TPackage extends IPackage> {
  (entryType: string, header: IHeader, pkg: null | TPackage, sources: Sources): boolean,
}
interface IValidateLink<TPackage extends IPackage> {
  (entryType: string, links: ILink[], header: IHeader, pkg: null | TPackage, sources: Sources): boolean,
}
interface IValidatePutPkg<TPackage extends IPackage> {
  (entryType: string): TPackage | void,
}
interface IValidateModPkg<TPackage extends IPackage> {
  (entryType: string): TPackage | void,
}
interface IValidateDelPkg<TPackage extends IPackage> {
  (entryType: string): TPackage | void,
}
interface IValidateLinkPkg<TPackage extends IPackage> {
  (entryType: string): TPackage | void,
}

/**
 * Zome optional callback interfaces
 */

interface IBridgeGenesis {
  (side: Integer, dna: Hash, appData: string): boolean,
}

interface IBundleCanceled<TBundleStartParam> {
  (reason: HCBundleCancelReason.UserCancel, userParam: TBundleStartParam): HCBundleCancelResponse, // FIXME: is this supposed to be spelled incorrectly?
}

interface IReceive<TReturn> {
  (from: Hash, message: string): TReturn,
}

/**
 * App globals
 */

declare const App: {
  Name: string,
  Agent: IAppAgent,
  DNA: IAppDNA,
  Key: IAppKey,
};

/**
 * System Constant globals
 */

declare const HC: {
  Version: string,
  Bridge: HCBridge,
  HashNotFound: HashNotFound,
  LinkAction: HCLinkAction,
  GetMask: HCGetMask,
  Migrate: HCMigrate,
  Status: HCStatus,
  SysEntryType: HCSysEntryType,
  PkgReq: {
    Chain: 'chain',
    ChainOpt: {
      None: 0,
      Headers: 1,
      Entries: 2,
      Full: 3,
    },
    EntryTypes: 'types',
  },
  BundleCancel: {
    Reason: HCBundleCancelReason,
    Response: HCBundleCancelResponse,
  },
};

/**
 * Global environment functions
 */

declare function debug(value: any): void;
declare function makeHash<TEntry>(entryType: string, entryData: TEntry): Hash | IError;
declare function property(name: string): string | IError;
declare function sign(doc: string): string | IError;
declare function verifySignature(signature: string, data: string, publicKey: string): boolean | IError;

declare function bridge<TReturn>(appDNAHash: Hash, zomeName: string, functionName: string, arguments: string): TReturn | IError; // FIXME: could return error?
declare function bridge<TArgs, TReturn>(appDNAHash: Hash, zomeName: string, functionName: string, arguments: TArgs): TReturn | IError; // FIXME: could return error?

declare function call<TReturn>(zomeName: string, functionName: string, arguments: string): TReturn | IError;
declare function call<TArgs extends object, TReturn>(zomeName: string, functionName: string, arguments: TArgs): TReturn | IError;

declare function send<TMessage extends object, TReturn>(to: Hash, message: TMessage, options?: ISendOptions): TReturn;

declare function getBridges(): Bridge[];

declare function getLinks(base: Hash, tag: string, options?: IGetLinksOptions): { Hash: Hash, }[] | IError;
declare function getLinks<TEntry>(base: Hash, tag: string, options?: IGetLinksOptionsLoad): IEntryWithHash<TEntry> | IError;

declare function get<TReturn>(hash: Hash, options?: IGetOptions): TReturn | HashNotFound;

declare function query(options?: IQueryOptionsHashes): IQueryResponseHash[] | IError;
declare function query(options?: IQueryOptionsHeaders): IQueryResponseHeader[] | IError;
declare function query(options?: IQueryOptionsHashes & IQueryOptionsHeaders): IError; // FIXME: what does it look like to return hashes and headers?
declare function query<TEntry>(options?: IQueryOptionsEntries): IQueryResponseEntry<TEntry>[] | IError;
declare function query<TEntry>(options?: IQueryOptionsEntries & IQueryOptionsHashes): IQueryResponseHashAndEntry<TEntry>[] | IError;
// declare function query<TEntry>(options?: IQueryOptionsEntries & IQueryOptionsHeaders): IError
// declare function query<TEntry>(options?: IQueryOptionsEntries & IQueryOptionsHashes & IQueryOptionsHeaders): IError

declare function commit(entryType: string, entryData: string): Hash | IError;
declare function commit(entryType: string, entryData: { Links: ILink[] }): Hash | IError;
declare function commit<TEntry>(entryType: string, entryData: TEntry): Hash | IError;
declare function update(entryType: string, entryData: string, replaces: Hash): Hash | IError;
declare function update<TEntry extends object>(entryType: string, entryData: TEntry, replaces: Hash): Hash | IError;

declare function updateAgent(options: IUpdateAgentOptions): Hash | IError;
declare function remove(entry: Hash, message?: string): Hash; // FIXME: message is optional?

declare function bundleStart<TBundleStartParam>(timeout: Integer, userParam: TBundleStartParam): void;
declare function bundleClose(commit: boolean): void;

/* eslint-enable */
