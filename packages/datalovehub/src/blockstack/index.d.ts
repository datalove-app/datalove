import * as IBlockstackID from './BlockstackID';

declare namespace blockstack {
  export type BlockstackID = IBlockstackID.T;

  export interface IProfile {}

  export interface IBlockstackProfile {
    lookupProfile(username: string, zoneFileLookupURL?: string): Promise<IProfile>,
  }

  export type IGetFileResponse = string | Buffer;
  export interface IGetFileOptions {
    app?: string,
    decrypt?: boolean,
    username?: string,
    zoneFileLookupURL?: string,
  }

  export type IPutFileContent = string | Buffer;
  export type IPutFileResponse = string | Buffer;
  export interface IPutFileOptions {
    encrypt?: boolean,
  }

  export interface IFileOptions extends IGetFileOptions, IPutFileOptions {}

  export interface IBlockstackStorage {
    getFile(path: string, options?: IGetFileOptions):
      Promise<IGetFileResponse>,
    putFile(path: string, content: IPutFileContent, options?: IPutFileOptions):
      Promise<IPutFileResponse>,
    getUserAppFileURL(path: string, username: string, appOrigin: string, zoneFileLookupURL?: string): Promise<string | null>,
    getAppBucketURL(gaiaHubURL: string, appPrivateKey: string): Promise<string>,
  }

  export type IBlockstack =
    & IBlockstackProfile
    & IBlockstackStorage;
}

export = blockstack;
