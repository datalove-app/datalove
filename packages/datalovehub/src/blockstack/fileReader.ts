import fromPromise from 'callbag-from-promise';
import { Reader } from 'fp-ts/lib/Reader';
// import { ReaderCallbag } from '../util/ReaderCallbag';

import {
  IBlockstackStorage,
  IGetFileOptions,
  IGetFileResponse,
  IPutFileContent,
  IPutFileOptions,
  IPutFileResponse,
} from '../../types/blockstack';

export type FileReader<T> = Reader<IBlockstackStorage, T>;
export { IGetFileOptions, IPutFileOptions } from '../../types/blockstack';

export interface IGetFile {
  (path: string, options: IGetFileOptions): FileReader<Promise<IGetFileResponse>>,
}

export interface IPutFile {
  (path: string, content: IPutFileContent, options: IPutFileOptions): FileReader<Promise<IPutFileResponse>>,
}

export const getFile: IGetFile = (path, options) => {
  return new Reader((storage: IBlockstackStorage) => {
    return storage.getFile(path, options);
  });
};

export const putFile: IPutFile = (path, content, options) => {
  return new Reader((storage: IBlockstackStorage) => {
    return storage.putFile(path, content, options);
  });
};

// export const getFile: IGetFile = (path, options) => {
//   return new ReaderCallbag((storage: IBlockstackStorage) => {
//     return fromPromise(storage.getFile(path, options));
//   });
// };
//
// export const putFile: IPutFile = (path, content, options) => {
//   return new ReaderCallbag((storage: IBlockstackStorage) => {
//     return fromPromise(storage.putFile(path, content, options));
//   });
// };
