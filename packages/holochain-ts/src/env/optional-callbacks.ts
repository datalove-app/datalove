/**
 * Zome optional callback interfaces
 */

/* eslint-disable space-infix-ops */

type TBridgeGenesis = (side: HCBridge, dna: Hash, appData: string) => boolean;

type TBundleCancelled<TBundleStartParam> = (reason: HCBundleCancelReason.UserCancel, userParam: TBundleStartParam) => HCBundleCancelResponse; // FIXME: is this supposed to be spelled incorrectly?

type TReceive<TMessage, TReturn> = (from: Hash, message: TMessage) => TReturn;

interface IZomeOptionalCallbacks<TBundleStartParam, TReceiveMessage, TReceiveResponse> {
  bridgeGenesis?: TBridgeGenesis,
  bundleCanceled?: TBundleCancelled<TBundleStartParam>,
  // bundleCanceled?: IBundleCancelled<TBundleStartParam, TResponse>,
  receive?: TReceive<TReceiveMessage, TReceiveResponse>,
  // receive?: (from: Hash, message: TReceiveMessage) => TReceiveResponse,
}

/* eslint-enable */
