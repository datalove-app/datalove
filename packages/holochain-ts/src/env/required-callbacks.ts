/**
 * Zome required callback interfaces
 */

/* eslint-disable space-infix-ops */

type TGenesis = () => boolean;
type TValidateCommit<TEntry, TPackage extends null | IBasePackageRequest = null> = (entryType: string, entry: TEntry, header: IHeader, pkg: IBasePackageRequest, sources: Sources) => boolean;
type TValidatePut<TEntry, TPackage extends null | IBasePackageRequest = null> = (entryType: string, entry: TEntry, header: IHeader, pkg: IBasePackageRequest, sources: Sources) => boolean;
type TValidateMod<TEntry, TPackage extends null | IBasePackageRequest = null> = (entryType: string, entry: TEntry, header: IHeader, replaces: Hash, pkg: IBasePackageRequest, sources: Sources) => boolean;
type TValidateDel<TPackage extends null | IBasePackageRequest = null> = (entryType: string, header: IHeader, pkg: TPackage, sources: Sources) => boolean;
type TValidateLink<TPackage extends null | IBasePackageRequest = null> = (entryType: string, links: ILink[], header: IHeader, pkg: TPackage, sources: Sources) => boolean;
type TValidatePutPkg<TPackage extends null | IBasePackageRequest = null> = (entryType: string) => TPackage | null;
type TValidateModPkg<TPackage extends null | IBasePackageRequest = null> = (entryType: string) => TPackage | null;
type TValidateDelPkg<TPackage extends null | IBasePackageRequest = null> = (entryType: string) => TPackage | null;
type TValidateLinkPkg<TPackage extends null | IBasePackageRequest = null> = (entryType: string) => TPackage | null;

interface IZomeRequiredCallbacks<TEntry, TPackage extends null | IBasePackageRequest> {
  genesis: TGenesis,
  validateCommit: TValidateCommit<TEntry, TPackage>,
  validatePut: TValidatePut<TEntry, TPackage>,
  validateMod: TValidateMod<TEntry, TPackage>,
  validateDel: TValidateDel<TPackage>,
  validateLink: TValidateLink<TPackage>,
  validatePutPkg: TValidatePutPkg<TPackage>,
  validateModPkg: TValidateModPkg<TPackage>,
  validateDelPkg: TValidateDelPkg<TPackage>,
  validateLinkPkg: TValidateLinkPkg<TPackage>,
}

/* eslint-enable */
