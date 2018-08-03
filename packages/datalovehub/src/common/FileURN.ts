import * as AppID from './core/AppID';
import * as MentionCount from './core/MentionCount';
import * as ObjectID from './core/ObjectID';
import * as IBlockstack from '../blockstack';
import * as BlockstackID from '../blockstack/BlockstackID';

/**
 * FileURN is a globally unique URN, found in the `id` and `url` properties of
 *  ActivityStreams Objects, and the `href` property of ActivityStreams Links
 * FileURN allows us to uniquely identify a file stored in anywhere in any
 *  user's Gaia hub
 *
 * a FileURN contains:
 *  - a prefix (`urn:datalovehub`)
 *  - the blockstack username of Gaia hub containing the file
 *  - the appID of the blockstack application
 *  - an objectID making up the start of the file path
 *  - [if mention ID] the mentionCount, making up the end of the file path
 */

export type T = string;
export interface IContext {
  currentUsername: IBlockstack.BlockstackID,
  currentAppID: string,
  currentAppURL: string,
}
export interface IOptions extends AppID.IOptions, ObjectID.IOptions {
  username?: IBlockstack.BlockstackID,
  mentionCount?: MentionCount.T,
}
export interface ISplitParts {
  prefix?: string,
  username: IBlockstack.BlockstackID,
  appID: AppID.T,
  objectID: ObjectID.T,
  mentionCount?: string,
}
export interface IPathParts {
  objectID: ObjectID.T,
  mentionCount?: string,
}

export const SEPARATOR = ':';
export const PREFIX = 'urn:datalovehub';

export const validateParts = ({ prefix, username, appID, objectID, mentionCount }: ISplitParts): boolean => {
  return ((typeof prefix === 'undefined') ? true : prefix === PREFIX)
    && BlockstackID.validate(username)
    && AppID.validate(appID)
    && ObjectID.validate(objectID)
    && ((typeof mentionCount === 'undefined')
      ? true
      : MentionCount.validateEncoded(mentionCount));
};

export const _combine = ({ username, appID, objectID, mentionCount }: ISplitParts): T => {
  return (typeof mentionCount === 'undefined')
    ? [PREFIX, username, appID, objectID].join(SEPARATOR)
    : [PREFIX, username, appID, objectID, mentionCount].join(SEPARATOR);
};

export const _split = (fileURN: T): ISplitParts => {
  const parts = fileURN.split(SEPARATOR);
  const [urn, datalove, username, appID, objectID, mentionCount = null] = parts;
  const prefix = `${urn}:${datalove}`;
  return (mentionCount === null)
    ? ({ prefix, username, appID, objectID })
    : ({ prefix, username, appID, objectID, mentionCount });
};

export const from = ({ username, mentionCount, ...options }: IOptions): T | null => {
  const appID = AppID.from(options);
  const objectID = ObjectID.from(options);
  const mentionCountStr = MentionCount
    .encode(mentionCount as MentionCount.T);

  const isValidUsername = BlockstackID.validate(username);
  return (!isValidUsername || appID === null || objectID === null)
    ? null
    : (mentionCountStr === null)
      ? _combine({ username, appID, objectID })
      : _combine({ username, appID, objectID, mentionCount: mentionCountStr });
};

export const fromDefaults = (options: IOptions, { currentAppID, currentAppURL, currentUsername }: IContext): T | null => {
  const defaults = {
    appID: currentAppID,
    appURL: currentAppURL,
    username: currentUsername,
  };
  return from(Object.assign(defaults, options));
};

export const to = (fileURN: T): IOptions | null => {
  const { mentionCount: mentionCountString, ...parts } = _split(fileURN);
  if (!validateParts(parts)) return null;

  const { prefix: _, ...newParts } = parts;
  const objectParts = ObjectID.to(newParts.objectID);
  const mentionCount = MentionCount.decode(mentionCountString as string);
  return (objectParts === null)
    ? null
    : (mentionCount === null)
      ? Object.assign(newParts, objectParts)
      : Object.assign(newParts, objectParts, { mentionCount });
};

const newFileURN = (_options: IOptions, context?: IContext): T | null => {
  const objectID = ObjectID.new();
  const options = Object.assign(_options, { objectID });
  return (typeof context === 'undefined')
    ? from(options)
    : fromDefaults(options, context);
};
export { newFileURN as new };
