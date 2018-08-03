import * as MentionCount from './MentionCount';
import * as ObjectID from './ObjectID';

/**
 * Similar to ObjectID, MentionID is an ObjectID concatenated with a 6-byte
 *  base64URL string of a 4-byte integer representing the nth mention
 *
 * a MentionID's ObjectID component is:
 *  - an ObjectID of an object (between 1 and 20-bytes)
 *  - an ObjectID of a URL (its the SHA256 hash of the normalized url
 *    (sans hash fragments and query params)
 */

export type T = string;
export interface IOptions extends IObjectIDOptions, IURLOptions {}
export interface IObjectIDOptions {
  objectID?: ObjectID.T | null,
  mentionCount?: MentionCount.T | null,
}
export interface IURLOptions {
  mentionURL?: string | null,
  mentionCount?: MentionCount.T | null,
}
export interface ISplitOptions {
  objectID: string,
  mentionCount: string,
}

export const MIN_LENGTH = ObjectID.MIN_LENGTH + MentionCount.LENGTH;
export const LENGTH = ObjectID.LENGTH + MentionCount.LENGTH;

export const FILENAME_MENTION_TEMPLATE =
  `^(?=^${ObjectID.PAD_TEMPLATE}*${ObjectID.CHAR_TEMPLATE}{${MIN_LENGTH},}$)`
    + `.{${LENGTH}}$`;
export const URL_MENTION_TEMPLATE = `^${ObjectID.CHAR_TEMPLATE}{${LENGTH}}$`;
export const PATTERN =
  new RegExp(`(${FILENAME_MENTION_TEMPLATE})|(${URL_MENTION_TEMPLATE})`);

const combine = (objectID: ObjectID.T, mentionCount: string): T =>
  objectID.concat(mentionCount);

const split = (mentionID: T): ISplitOptions => ({
  objectID: mentionID.slice(0, -MentionCount.LENGTH),
  mentionCount: mentionID.substr(-MentionCount.LENGTH),
});

export const validate = (mentionID: T | any): boolean =>
  typeof mentionID === 'string'
    && PATTERN.test(mentionID);

export const encode = (
  objectID: ObjectID.T,
  mentionCount: number = 1,
): T | null => {
  if (!ObjectID.validate(objectID)) return null;

  const countString = MentionCount.encode(mentionCount);
  if (countString === null) return null;

  return combine(objectID as string, countString);
};

export const decode = (mentionID: T): IObjectIDOptions | null => {
  const { objectID, mentionCount: mentionCountString } = split(mentionID);
  const mentionCount = MentionCount.decode(mentionCountString);
  return (mentionCount === null || !ObjectID.validate(objectID))
    ? null
    : { objectID, mentionCount };
};

const newMentionID = (): T | null => encode(ObjectID.new(), 1);
export { newMentionID as new };

export const incrementMentionID = (mentionID: T): T | null => {
  const parts = decode(mentionID);
  if (parts === null) return null;

  const { objectID, mentionCount } = parts;
  if (((mentionCount as MentionCount.T) + 1) > MentionCount.MAX) return null;

  return encode(objectID as ObjectID.T, (mentionCount as MentionCount.T) + 1);
};

export const fromURL = (url: string, mentionCount: number = 1): T | null => (
  typeof url !== 'string'
    ? null
    : encode(ObjectID.fromURL(url) as ObjectID.T, mentionCount)
);

export const toFileName = (mentionID: T): string | null => {};

export const toURL = (mentionID: T): IURLOptions | null => {
  const parts = decode(mentionID);
  if (parts === null) return null;

  return Object.assign(parts, {
    mentionURL: ObjectID.toURL(parts.objectID as string),
  });
};

export const from = (options?: IOptions): T | null => {
  if (typeof options === 'undefined') {
    return newMentionID();
  }

  const { objectID, mentionCount = 1, mentionURL } = options;

  if (typeof mentionURL === 'string') {
    return fromURL(mentionURL, mentionCount as number);
  } else if (typeof objectID === 'string') {
    return encode(objectID, mentionCount as number);
  }

  return null;
};
