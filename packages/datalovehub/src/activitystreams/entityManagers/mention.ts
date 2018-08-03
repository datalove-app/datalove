import map from 'callbag-map';
import pipe from 'callbag-pipe';
import { getFile, putFile } from '../blockstack/fileReader';

import { FileReader, IGetFileOptions, IPutFileOptions } from '../blockstack/fileReader';

export interface MentionManagerOptions {
  appPath: string,
  folderName: string,
}

export interface ILinkContent {}

export default class MentionManager {
  public appPath: string;
  public folderName: string;

  public constructor(options: MentionManagerOptions) {
    this.appPath = options.appPath;
    this.folderName = options.folderName;
  }

  public path(targetID: string): string {
    return `${this.appPath}${this.folderName}/${targetID}`;
  }

  public get(targetID: string, options: IGetFileOptions): FileReader<ILinkContent> {
    const path = this.path(targetID);
    return getFile(path, options)
      .map(file => (typeof file === 'string' ? JSON.parse(file) : file));
  }

  public put(targetID: string, linkContent: ILinkContent, options: IPutFileOptions): FileReader<any> {
    const path = this.path(targetID);
    const contentString = JSON.stringify(linkContent);
    return putFile(path, contentString, options);
  }

  public update(targetID: string, linkContent: ILinkContent, getOptions: IGetFileOptions, putOptions: IPutFileOptions): FileReader<any> {
    const path = this.path(targetID);
    return this.get(path, getOptions)
      .chain(_ => this.put(targetID, linkContent, putOptions));
  }

  public upsert(targetID: string, linkContent: ILinkContent, options: IPutFileOptions): FileReader<any> {
    return this.put(targetID, linkContent, options);
  }

  public appendTo(targetID: string, linkContent: ILinkContent, getOptions: IGetFileOptions, putOptions: IPutFileOptions): FileReader<any> {
    // get file
    // add links
    // call putReader with new links

    return this.get(targetID, getOptions)
      .map(file$ => pipe(
        file$,
        map((links: Array<ILinkContent>) => links.concat(linkContent))
      ))
      .chain((links: Array<ILinkContent>) => this.put(targetID, links, putOptions));
  }
}
