import {
  FileReader,
  IGetFileOptions,
  getFile,
  putFile,
} from '../blockstack/fileReader';

// import { ReaderCallbag } from '../util/ReaderCallbag';
import { IBlockstackStorage } from '../../types/blockstack';

type PageNumber = string | number;
type URI = string;
type SortOrder = 'ASC' | 'DESC';

interface IObject {
  '@context': string,
  id: URI,
  type: string[],
  url: URI,
}

interface ICollectionIndex extends IObject {}
interface ICollectionPage extends IObject {
  items: Array<any>,
}
interface ICollectionPages {
  [pageNumber: string]: ICollectionPage,
}
interface IDirtyFiles {
  [fileName: string]: ICollectionIndex | ICollectionPage,
}

const getIndexFileName = () => 'index.json';
const getPageFileName = (pageNum: PageNumber): string => `page${pageNum}.json`;

export default class CollectionManager {
  public object_id: string;
  public id: URI;
  public url: URI;
  public index: ICollectionIndex;
  public pages: ICollectionPages;

  private _dirtyIndex: boolean = false;
  private _dirtyPages: { [pageNumber: PageNumber]: boolean, } = {};

  public get getDirtyFiles(): IDirtyFiles {
    const indexFileName = getIndexFileName();
    const files: IDirtyFiles = {};

    if (this._dirtyIndex) {
      files[indexFileName] = (this.index as ICollectionIndex);
    }

    Object.keys(this._dirtyPages).forEach((pageNumber: number) => {
      if (this._dirtyPages[pageNumber]) {
        const page: ICollectionPage = this.pages[pageNumber];
        const pageFileName = getPageFileName(pageNumber);
        files[pageFileName] = page;
      }
    });

    return files;
  }

  private set setPage(page: ICollectionPage, pageNumber: PageNumber = 1) {
    this.pages[pageNumber] = page;
  }

  private set markIndexAsDirty(): void {
    this._dirtyIndex = true;
  }

  private set markPagesAsDirty(pageNumbers: PageNumber | Array<PageNumber>): void {
    if (Array.isArray(pageNumbers)) {
      pageNumbers.forEach((num: PageNumber) => {
        this._dirtyPages[num] = true;
      });
    } else {
      this._dirtyPages[pageNumber] = true;
    }
  }

  constructor(index: ICollectionIndex, pages: ICollectionPages) {
    this.id = index.id;
    this.url = index.url;
    this.index = index;
    this.pages = pages;
  }

  update(fn: (collection: CollectionManager) => {}): void {
    const originalIndex = this.index;
    const originalPages = this.pages;

    const { index, pages } = fn(this);
    this.index = index;
    this.pages = pages;

    if (this.index !== originalIndex) {
      this.markIndexAsDirty();
    }

    Object.keys(this.pages).forEach((pageNumber: PageNumber) => {
      if (this.pages[pageNumber] === originalPages[pageNumber]) {
        this.markPagesAsDirty(pageNumber);
      }
    });
  }
}

function serializeIndexFile() {}

function serializePageFile() {}

function parseIndexFile(indexFile: string): ICollectionIndex {
  return JSON.parse(indexFile);
}

function parsePageFile(pageFile: string): ICollectionPage {
  return JSON.parse(pageFile);
}

function pathFromURN(urn: URI): string {
  const parts = urn.split(':');
  return parts.slice(2).join('/');
}

function indexPathFromURN(urn: URI): string {
  const path = pathFromURN(urn);
  const indexFileName = getIndexFileName();
  return `${path}/${indexFileName}`;
}

function pagePathFromURN(urn: URI, pageNumber: PageNumber = 1): string {
  const path = pathFromURN(urn);
  const pageFileName = getPageFileName(pageNumber);
  return `${path}/${pageFileName}`;
}

/******************************************************************************
 *
 ******************************************************************************/

function newCollection(id: string, type: string[], urn: URI): CollectionManager {
  const index: ICollectionIndex = {
    id,
    type,
    url: urn,
  };
  const pageNumber = 1;
  const page: ICollectionPage = {};
  const collection = new CollectionManager(index, { [pageNumber]: page });
  collection.markIndexAsDirty();
  collection.markPagesAsDirty(1);
  return collection;
}

function createCollection(collectionID: URI) {
  return getCollection(collectionID)

}

function getCollection(collectionID: URI, pageNumber: PageNumber = 1, options: IGetFileOptions = {}): ReaderCallbag<IBlockstackStorage, CollectionManager> {
  return getIndex(collectionID, options)
    .chain((index: ICollectionIndex) => {
      return getPage(collectionID, pageNumber, options)
        .map(page => new CollectionManager(index, { [pageNumber]: page }));
    });
}

function getCollectionItems(collectionID: URI, cursor: number, limit: number, sortOrder: SortOrder): ReaderCallbag<IBlockstackStorage, CollectionManager> {

}

function getIndex(collectionID: URI, options: IGetFileOptions = {}): FileReader<ICollectionIndex> {
  const path = indexPathFromURN(collectionID);
  return getFile(path, options)
    .map(parseIndexFile);
}

function getPage(collectionID: URI, page: PageNumber = 1, options: IGetFileOptions = {}): ReaderCallbag<IBlockstackStorage, ICollectionPage> {
  const path = pagePathFromURN(collectionID, page);
  return getFile(path, options)
    .map(parsePageFile);
}

function flush(reader: ReaderCallbag<IBlockstackStorage, CollectionManager>): ReaderCallbag<IBlockstackStorage, any> {
  // grab dirty files
  // create and merge a number of putFile readers
  // return the reader
  return reader
    .chain((collection: CollectionManager) => {
      const dirtyFiles = collection.getDirtyFiles();
      return putFile();
    });
}
