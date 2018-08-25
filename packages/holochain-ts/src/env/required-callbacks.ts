/**
 * Zome required callback interfaces
 */

interface IGenesis {
  (): boolean,
}
interface IValidateCommit<TEntry, TPackage extends null | IPackage> {
  (entryType: string, entry: TEntry, header: IHeader, pkg: TPackage, sources: Sources): boolean,
}
interface IValidatePut<TEntry, TPackage extends null | IPackage> {
  (entryType: string, entry: TEntry, header: IHeader, pkg: TPackage, sources: Sources): boolean,
}
interface IValidateMod<TEntry, TPackage extends null | IPackage> {
  (entryType: string, entry: TEntry, header: IHeader, replaces: Hash, pkg: TPackage, sources: Sources): boolean,
}
interface IValidateDel<TPackage extends null | IPackage> {
  (entryType: string, header: IHeader, pkg: TPackage, sources: Sources): boolean,
}
interface IValidateLink<TPackage extends null | IPackage> {
  (entryType: string, links: ILink[], header: IHeader, pkg: TPackage, sources: Sources): boolean,
}
interface IValidatePutPkg<TPackage extends null | IPackage> {
  (entryType: string): TPackage | null,
}
interface IValidateModPkg<TPackage extends null | IPackage> {
  (entryType: string): TPackage | null,
}
interface IValidateDelPkg<TPackage extends null | IPackage> {
  (entryType: string): TPackage | null,
}
interface IValidateLinkPkg<TPackage extends null | IPackage> {
  (entryType: string): TPackage | null,
}

interface IZomeRequiredCallbacks<TEntry, TPackage extends null | IPackage> {
  genesis: IGenesis,
  validateCommit: IValidateCommit<TEntry, TPackage>,
  validatePut: IValidatePut<TEntry, TPackage>,
  validateMod: IValidateMod<TEntry, TPackage>,
  validateDel: IValidateDel<TPackage>,
  validateLink: IValidateLink<TPackage>,
  validatePutPkg: IValidatePutPkg<TPackage>,
  validateModPkg: IValidateModPkg<TPackage>,
  validateDelPkg: IValidateDelPkg<TPackage>,
  validateLinkPkg: IValidateLinkPkg<TPackage>,
}
