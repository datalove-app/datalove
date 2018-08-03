import { GraphQLResolveInfo } from 'graphql';
import makeCreateOperation from './create';
import Context = require('../../../common/context');

export interface IEntityOperationTask {
  name: string,
  perform: () => Promise<any>,
  rollback: () => Promise<any>,
}
export interface IEntityTaskResolver<IArgs> {
  (root: any,
   args: IArgs,
   context: Context.IContext,
   info: GraphQLResolveInfo,
  ): Promise<{
    task: IEntityOperationTask,
    finalResolver?: () => Promise<any>,
  }>,
}
export interface IEntityOperation<IArgs> {
  resolver: IEntityTaskResolver<IArgs>,
  exportName?: string,
}
export interface IEntityOperationInput {
  [argument: string]: any,
}
export interface IEntityOperationCreator {
  <IArgs>(input: IEntityOperationInput): IEntityOperation<IArgs>,
}

/**
 * notes:
 *
 * each operation either:
 *  (for files, objects, collections, and links)
 *  - create: creates a brand new entity
 *  - update: updates an existing entity
 *  - upsert: upserts an entity
 *  (for collections):
 *  - appendLink: appends a Link to an collection
 *  (for objects):
 *  - updateLink: updates an exisitng Link on an object
 *
 * then (optionally):
 *  - exports from previous steps
 * then finally:
 *  - returns a promise that resolves to the updated in-memory object
 *
 *
 *
 * exportLock(to)
 *  - lock before retrieving from exportMap (or network if missing)
 *  - release after using it
 * exportLock(exportName)
 *  - lock before retrieving from exportMap
 *  - release after using it
 * entityLock(fileURN)
 *  - lock before retrieving from entityMap (or network if missing)
 *  - release after committing
 */

export { makeCreateOperation };
