import * as AppID from './core/AppID';
import * as ObjectID from './core/ObjectID';

/**
 * ObjectMemo is 26-byte string composed of:
 *  - an AppID, denoting the application Gaia "bucket" that hosts the object
 *  - an ObjectID of the object containing data related to the Stellar
 *    transaction
 */

export type T = string;
export interface IOptions extends AppID.IOptions, ObjectID.IOptions {}
export interface ISplitParts {
  appID: AppID.T,
  objectID: ObjectID.T,
}

export const LENGTH = AppID.LENGTH + ObjectID.LENGTH;

export const validateParts = ({ appID, objectID }: ISplitParts) =>
  AppID.validate(appID) && ObjectID.validate(objectID);

export const _combine = (parts: ISplitParts): T =>
  parts.appID.concat(parts.objectID);

export const _split = (memo: T): ISplitParts => ({
  appID: memo.slice(0, AppID.LENGTH),
  objectID: memo.substr(AppID.LENGTH, ObjectID.LENGTH),
});

export const from = (options: IOptions): T | null => {
  const appID = AppID.from(options);
  const objectID = ObjectID.from(options);
  return (appID === null || objectID === null)
    ? null
    : _combine({ appID, objectID });
};

export const to = (memo: T): IOptions | null => {
  const parts = _split(memo);
  const objectParts = ObjectID.to(parts.objectID);
  return (AppID.validate(parts.appID) && objectParts !== null)
    ? Object.assign(objectParts, { appID: parts.appID })
    : null;
};
