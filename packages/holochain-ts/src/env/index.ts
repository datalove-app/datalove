// Type definitions for holochain 0.0.1
// Project: https://github.com/datalove-app/js-datalove

/* eslint-disable indent, no-restricted-globals, no-unused-vars, space-infix-ops */


type IBridge = IBridgeCaller | IBridgeCallee;
type DateTime = string; // looks like 2018-03-21T11:37:15Z
type Either<TValue, TError extends IError = IError> = TValue | IError;
type Integer = number;
type Hash = string;
type HashNotFound = null;
type LinkTag = string;
type Sources = string[];
type Token = string;

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

interface IError {
  name: 'HolochainError',
}

interface IHash {
  Hash: Hash,
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

interface ILinksEntry {
  Links: ILink[],
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

/* eslint-enable */
