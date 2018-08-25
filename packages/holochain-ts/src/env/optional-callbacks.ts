/**
 * Zome optional callback interfaces
 */

interface IBridgeGenesis {
  (side: HCBridge, dna: Hash, appData: string): boolean,
}

interface IBundleCancelled<TBundleStartParam> {
  (reason: HCBundleCancelReason.UserCancel, userParam: TBundleStartParam): HCBundleCancelResponse, // FIXME: is this supposed to be spelled incorrectly?
}

interface IReceive<TReturn> {
  (from: Hash, message: string): TReturn,
}

interface IZomeOptionalCallbacks<TBundleStartParam, TReceiveResponse> {
  bridgeGenesis?: IBridgeGenesis,
  bundleCanceled?: IBundleCancelled<TBundleStartParam>,
  receive?: IReceive<TReceiveResponse>,
}
