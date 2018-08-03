import makeCreateOperation from './base/create';
import makeUpdateOperation from './base/update';
import makeUpsertOperation from './base/upsert';
import makeUpdateLinkOperation from './base/updateLink';
import ObjectManager from '../entityManagers/object';

export const create = makeCreateOperation(ObjectManager);
export const update = makeUpdateOperation(ObjectManager);
export const upsert = makeUpsertOperation(ObjectManager);
export const updateLink = makeUpdateLinkOperation(ObjectManager);
