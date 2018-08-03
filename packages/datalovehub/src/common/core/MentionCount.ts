import base64URL from 'urlsafe-base64';
import { CHAR_TEMPLATE } from './ObjectID';

export type T = number;

export const BYTE_LENGTH = 4;
export const LENGTH = 6;
export const PATTERN = new RegExp(`/^${CHAR_TEMPLATE}{${LENGTH}}$/`);
export const MIN_COUNT = 0;
export const MAX_COUNT = (2 ** (BYTE_LENGTH * 4)) - 1;

const newUInt32Buffer = (num: number) => {
  const buf = Buffer.allocUnsafe(BYTE_LENGTH);
  buf.writeUInt32BE(num, 0);
  return buf;
};

export const validateDecoded = (count: T | any): boolean =>
  Number.isInteger(count)
    && count > MIN_COUNT
    && count <= MAX_COUNT;

export const validateEncoded = (count: string | any): boolean =>
  typeof count === 'string'
    && PATTERN.test(count);

export const encode = (count: T): string | null => (
  validateDecoded(count)
    ? base64URL.encode(newUInt32Buffer(count))
    : null
);

export const decode = (countString: string): T | null => (
  !validateEncoded(countString)
    ? null
    : base64URL
      .decode(countString)
      .readUInt32BE(0)
);
