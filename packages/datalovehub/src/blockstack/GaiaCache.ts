import LRU from 'lru-cache';

import * as ILRU from '../../../types/lru-cache';
import {
  IBlockstackStorage,
  IGetFileOptions,
  IGetFileResponse,
  IPutFileContent,
  IPutFileOptions,
  IPutFileResponse,
} from '../../../types/blockstack';

const CONSTANT_LENGTH = () => 1;

// implementation, move this elsewhere
export default class GaiaCache implements IBlockstackStorage {
  private _blockstack: IBlockstackStorage;
  private _cache: ILRU.ILRU;

  public constructor(blockstackClient: IBlockstackStorage, cacheOptions: ILRU.Options) {
    this._blockstack = blockstackClient;
    this._cache = (LRU as ILRU.LRU)(Object.assign({}, cacheOptions, {
      max: 100,
      length: CONSTANT_LENGTH,
    }));
  }

  public async getFile (path: string, options: IGetFileOptions): Promise<IGetFileResponse> {
    const username = options.username || null;
    const app = options.app || null;

    if (username === null && app === null) {
      // only hit the cache for users' own files for this app
      if (this._cache.has(path)) {
        return this._cache.get(path);
      }
    }

    const content = await this._blockstack.getFile(path, options);
    this._cache.set(path, content);
    return content;
  }

  public async putFile(path: string, content: IPutFileContent, options: IPutFileOptions): Promise<IPutFileResponse> {
    const result = await this._blockstack.putFile(path, content, options);
    this._cache.set(path, content);
    return result;
  }
}
