import { IBlockstack, IFileOptions, IGetFileOptions } from '../../blockstack';
import * as FileURN from '../../common/FileURN';
import { IContext } from '../../common/context';
import * as AppID from '../../common/core/AppID';
import { ILock } from '../../util/promise-lock';
import {IPathParts} from "../../common/FileURN";

export type SerializedState =
  | string
  | Buffer;
export interface SerializedStates {
  [fileName: string]: SerializedState,
}
export type IEntitiesMap = Map<string, IEntityManager<any, IContext, { [arg: string]: any, }>>;
export interface ITransactionManager {
  currentUsername: IBlockstack.BlockstackID,
  currentAppID: string,
  currentAppURL: string,
  appIDCache: AppID.IAppIDCache,
  blockstack: IBlockstack,
  entities: IEntitiesMap,
  entityLock: ILock,

  newExportsMap(): IEntitiesMap,
  newExportsLock(): ILock,
  newOperationID(): string,
}
export interface IBlockstackFileDescriptor {
  username: IBlockstack.BlockstackID,
  appID: AppID.T,
  appOrigin: string,
  path: string,
  options?: IBlockstack.IFileOptions,
}

export interface IEntityManagerStatic<IState, OState, View> {
  getByName(name: string, options: IGetFileOptions, TransactionManager: ITransactionManager): Promise<IEntityManager>,
  getByFileURN(fileURN: string, options: IGetFileOptions, TransactionManager: ITransactionManager): Promise<IEntityManager>,
  getByBlockstackFile(file: IBlockstackFileDescriptor, TransactionManager: ITransactionManager): Promise<IEntityManager>,

  create(nameOrFileURN: null | string | FileURN.T, options: IFileOptions, TransactionManager: ITransactionManager): Promise<IEntityManager>,
  createNew(options: IBlockstack.IPutFileOptions, TransactionManager: ITransactionManager): Promise<IEntityManager>,
  createByName(name: string, options: IFileOptions, TransactionManager: ITransactionManager): Promise<IEntityManager>,
  createByFileURN(fileURN: FileURN.T, options: IFileOptions, TransactionManager: ITransactionManager): Promise<IEntityManager>,
  createByBlockstackFile(file: IBlockstackFileDescriptor, TransactionManager: ITransactionManager): Promise<IEntityManager>,

  updateByName(name: string, options: IFileOptions, TransactionManager: ITransactionManager): Promise<IEntityManager>,
  updateByFileURN(fileURN: FileURN.T, options: IFileOptions, TransactionManager: ITransactionManager): Promise<IEntityManager>,
  updateByBlockstackFile(file: IBlockstackFileDescriptor, TransactionManager: ITransactionManager): Promise<IEntityManager>,

  upsertByName(name: string, options: IBlockstack.IPutFileOptions, TransactionManager: ITransactionManager): Promise<IEntityManager>,
  upsertByFileURN(fileURN: FileURN.T, options: IBlockstack.IPutFileOptions, TransactionManager: ITransactionManager): Promise<IEntityManager>,
  upsertByBlockstackFile(file: IBlockstackFileDescriptor, TransactionManager: ITransactionManager): Promise<IEntityManager>,
}

export interface IEntityManager<IState, OState, View> {
  id(): FileURN.T,
  blockstackFile(): IBlockstackFileDescriptor,

  update(fn: (file: OState) => OState): Promise<this>,
  view(): Promise<View>,

  commit(): Promise<void>,
  rollback(): Promise<void>,
}

// Blockstack-related operations

export const _combinePath = (parts: IPathParts): string => {
  return (typeof parts.mentionCount === 'string')
    ? `${parts.objectID}/${parts.mentionCount}`
    : parts.objectID;
};

export const _splitPath = (path: string): IPathParts => {
  const [objectID, mentionCountString = ''] = path.split('/');
  return mentionCountString === ''
    ? { objectID }
    : { objectID, mentionCount: mentionCountString };
};

export const setBlockstackFileDefaults = (file: IBlockstackFileDescriptor, TransactionManager: ITransactionManager): IBlockstackFileDescriptor => {
  const { appIDCache, currentAppID, currentAppURL, currentUsername } =
    TransactionManager;
  let { appID, appOrigin, username } = file;

  const missingAppID = typeof appID === 'undefined' || appID === null;
  const missingAppURL = typeof appOrigin === 'undefined' || appOrigin === null;
  const missingUsername = typeof username === 'undefined';

  if (missingAppID && missingAppURL) {
    appID = currentAppID;
    appOrigin = currentAppURL;
  } else if (missingAppID) {
    appID = (AppID.fromURL(appOrigin as string) as AppID.T);
  } else if (missingAppURL) {
    appOrigin = appIDCache.getByID(appID as AppID.T);
  }

  if (missingUsername) {
    username = currentUsername;
  }

  return Object.assign({}, file, {
    appID,
    appOrigin,
    username,
  });
};

export const blockstackFileToFileURN = (_file: IBlockstackFileDescriptor, TransactionManager: ITransactionManager): [FileURN.T, IBlockstack.IFileOptions] | null => {
  const file = setBlockstackFileDefaults(_file, TransactionManager);

  const { username, appID, path, options = {} } = file;
  const { objectID, mentionCount } = _splitPath(path);
  const parts = { username, appID, objectID, mentionCount };
  return FileURN.validateParts(parts)
    ? [FileURN._combine(parts), options]
    : null;
};

export const fileURNToBlockstackFile = (fileURN: FileURN.T, options: IBlockstack.IFileOptions, TransactionManager: ITransactionManager): IBlockstackFileDescriptor | null => {
  const parts = FileURN._split(fileURN);
  if (!FileURN.validateParts(parts)) return null;

  const { username, appID, objectID, mentionCount } = parts;
  const path = _combinePath({ objectID, mentionCount });
  const file = { username, appID, path, options };
  return setBlockstackFileDefaults(file as IBlockstackFileDescriptor, TransactionManager);
};

// export interface IEntityManagerStatic<IState, OState, View> {
//   /**
//    * Creates new entity with a random ID
//    *
//    * @param {IState} state
//    * @param {} blockstack
//    * @param {} options
//    * @returns {Promise<IEntityManager<IState, OState, View>>}
//    */
//   new(state: IState, blockstack: IBlockstackStorage, options?: IPutFileOptions): Promise<IEntityManager<IState, OState, View>>,
//
//   /**
//    * Creates entity with the given ID, fails if it already exists
//    *
//    * @param {T} fileURN
//    * @param {IState} state
//    * @param {} blockstack
//    * @param {} options
//    * @returns {Promise<IEntityManager<IState, OState, View>>}
//    */
//   create(fileURN: FileURN.T, state: IState, blockstack: IBlockstackStorage, options?: IPutFileOptions): Promise<IEntityManager<IState, OState, View>>,
//
//   /**
//    * Fetches entity with the given ID
//    *
//    * @param {T} fileURN
//    * @param {} blockstack
//    * @param {} options
//    * @returns {Promise<IEntityManager<IState, OState, View>>}
//    */
//   fromFileURN(fileURN: FileURN.T, blockstack: IBlockstackStorage, options?: IGetFileOptions): Promise<IEntityManager<IState, OState, View>>,
//
//   // encoding ops
//   // serialize?(state: OState): SerializedStates,
//   // parse?(serialized: SerializedStates): OState,
// }
//
// export interface IEntityManager<IState, OState, View> {
//   _state: OState,
//
//   constructor(blockstack: IBlockstackStorage, options?: IPutFileOptions): void,
//
//   // local read ops
//   id(): string,
//   url(): string,
//   view(...options: any[]): Promise<View>,
//
//   // local write ops
//   update(fn: (file: OState) => OState): Promise<this>,
//
//   // network write ops
//   commit(): Promise<any>,
// }
