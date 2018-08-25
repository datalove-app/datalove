/* eslint-disable indent, no-restricted-globals, no-unused-vars, space-infix-ops */

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
  Bridge: {
    Caller: HCBridge.Caller,
    Callee: HCBridge.Callee,
  },
  HashNotFound: HashNotFound,
  LinkAction: {
    Add: HCLinkAction.Add,
    Del: HCLinkAction.Del,
  },
  GetMask: {
    Default: HCGetMask.Default,
    Entry: HCGetMask.Entry,
    EntryType: HCGetMask.EntryType,
    Sources: HCGetMask.Sources,
    All: HCGetMask.All,
  },
  Migrate: {
    Close: HCMigrate.Close,
    Open: HCMigrate.Open,
  },
  Status: {
    Live: HCStatus.Live,
    Rejected: HCStatus.Rejected,
    Deleted: HCStatus.Deleted,
    Modified: HCStatus.Modified,
    Any: HCStatus.Any,
  },
  SysEntryType: {
    DNA: HCSysEntryType.DNA,
    Agent: HCSysEntryType.Agent,
    Key: HCSysEntryType.Key,
    Headers: HCSysEntryType.Headers,
    Del: HCSysEntryType.Del,
    Migrate: HCSysEntryType.Migrate,
  },
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

/* eslint-enable */
