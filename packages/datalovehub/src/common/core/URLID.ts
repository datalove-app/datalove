import crypto = require('crypto');
import base64URL from 'urlsafe-base64';

/**
 * Base64URL SHA256 of an arbitrary-length string
 *  - can truncate hash to specified byte-length (from start)
 */

export type T = string;
export interface INormalize {
  (url: string): string,
}
export interface IEncode {
  (
    _url: string,
    length: number,
    byteLength: number,
    normalizeFn: INormalize
  ): T | null,
}
export interface IOptions {
  url?: string | null,
  urlID?: T | null,
}

export const DEFAULT_NORMALIZE: INormalize = url => url;

export const validate = (urlID: T | any, pattern: RegExp): boolean =>
  typeof urlID === 'string'
    && pattern.test(urlID);

export const verify = (url: string, objectID: T, _encode: (url: string) => T | null): boolean =>
  _encode(url) === objectID;

export const encode: IEncode = (_url, length, byteLength, normalizeFn = DEFAULT_NORMALIZE): T | null => {
  const url = normalizeFn(_url);
  const hash = crypto.createHash('sha256');
  hash.update(url, 'utf8');
  const buf = hash
    .digest()
    .slice(0, byteLength);
  const urlID = base64URL.encode(buf);

  return (urlID.length !== length)
    ? null
    : urlID;
};

export const from = (options: IOptions, _validate: (urlID: T | any) => boolean, _encode: (url: string) => T | null): T | null => {
  const { urlID = null, url = null } = options;

  if (typeof urlID === 'string' && _validate(urlID as string)) {
    return urlID;
  } else if (typeof url === 'string') {
    return _encode(url as string);
  }

  return null;
};
