export type T = string;

const REGEX = /\w{0,}\.id$/;

export const validate = (blockstackID: T): boolean =>
  REGEX.test(blockstackID);
