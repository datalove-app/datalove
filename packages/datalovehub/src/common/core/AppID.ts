import * as URLID from './URLID';
import { CHAR_TEMPLATE } from './ObjectID';

export type T = string;
export interface IOptions {
  appID?: T,
  appURL?: string,
}
export interface IAppIDCache {
  getByID(id: T): string,
  getByURL(url: string): T,
  setByID(id: T, url: string): any,
  setByURL(url: string, id: T): any,
}

export const BYTE_LENGTH = 4;
export const LENGTH = 6;
export const PATTERN = new RegExp(`/^${CHAR_TEMPLATE}{${LENGTH}}$/`);

export const validate = (urlID: T | any): boolean =>
  URLID.validate(urlID, PATTERN);

export const fromURL = (appURL: string): T | null =>
  URLID.encode(appURL, LENGTH, BYTE_LENGTH, URLID.DEFAULT_NORMALIZE);

export const verify = (appURL: string, appID: T): boolean =>
  URLID.verify(appURL, appID, fromURL);

export const from = ({ appURL, appID }: IOptions): T | null =>
  URLID.from({ url: appURL, urlID: appID }, validate, fromURL);
