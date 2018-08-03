import generate from 'nanoid/generate';
import * as URLID from './URLID';

/**
 * ObjectID is:
 *  - a globally-unique 20-byte NanoID
 *  - a string of max 20 base64URL characters, left-padded to 20-bytes with `=`
 *  - a URLID of max 20 characters
 */

export type T = string;
export interface IOptions {
  objectID?: T,
  fileName?: string,
  objectURL?: string,
}

export const ALPHABET =
  '_-0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';
export const BYTE_LENGTH = 15;
export const MIN_LENGTH = 1;
export const LENGTH = 20;

export const CHAR_TEMPLATE = '[A-Za-z0-9_-]';
export const PAD_TEMPLATE = '=';
export const FILENAME_TEMPLATE =
  `^${CHAR_TEMPLATE}{${MIN_LENGTH},${LENGTH - 1}}$`;
export const FILENAME_ID_TEMPLATE =
  `^(?=^${PAD_TEMPLATE}+${CHAR_TEMPLATE}+$).{${LENGTH}}$`;
export const URL_ID_TEMPLATE =
  `^${CHAR_TEMPLATE}{${LENGTH}}$`;

export const PAD_PATTERN = new RegExp(`^${PAD_TEMPLATE}*`);
export const FILENAME_PATTERN = new RegExp(FILENAME_TEMPLATE);
export const FILENAME_ID_PATTERN = new RegExp(FILENAME_ID_TEMPLATE);
export const URL_ID_PATTERN = new RegExp(URL_ID_TEMPLATE);
export const PATTERN = new RegExp(`(${FILENAME_ID_TEMPLATE})|(${URL_ID_TEMPLATE})`);

export const normalize = (url: string): string => url;

// Validation

export const validate = (objectID: T | any): boolean =>
  typeof objectID === 'string'
    && PATTERN.test(objectID);

export const validateURLID = (objectID: T | any): boolean =>
  typeof objectID === 'string'
    && URL_ID_PATTERN.test(objectID);

export const validateFileNameID = (objectID: T | any): boolean =>
  typeof objectID === 'string'
    && FILENAME_ID_PATTERN.test(objectID);

export const validateFileName = (fileName: string | any): boolean =>
  typeof fileName === 'string'
    && FILENAME_PATTERN.test(fileName);

// Creation, Serialization

const newObjectID = (): T => generate(ALPHABET, LENGTH);
export { newObjectID as new };

export const fromFileName = (fileName: string): T | null => (
  validateFileName(fileName)
    ? null
    : fileName.padStart(LENGTH, PAD_TEMPLATE)
);

export const fromURL = (url: string, normalizeFn: URLID.INormalize = normalize): T | null =>
  URLID.encode(url, LENGTH, BYTE_LENGTH, normalizeFn);

export const from = (options?: IOptions): T | null => {
  if (typeof options === 'undefined') {
    return newObjectID();
  }

  const { objectURL, fileName, objectID } = options;
  if (typeof objectURL === 'string') {
    return fromURL(objectURL);
  } else if (validateFileName(fileName)) {
    return fromFileName(fileName as string);
  } else if (typeof objectID === 'string' && validate(objectID)) {
    return objectID;
  }

  return null;
};

// Parsing

export const toFileName = (objectID: T): string | null => (
  validateFileNameID(objectID)
    ? objectID.replace(PAD_PATTERN, '')
    : null
);

export const to = (objectID: T): IOptions | null => {
  if (validateFileNameID(objectID)) {
    const fileName = toFileName(objectID);
    return fileName === null
      ? null
      : { objectID, fileName };
  } else if (validateURLID(objectID)) {
    return { objectID };
  }

  return null;
};

export const verify = (url: string, objectID: T): boolean =>
  fromURL(url) === objectID;
