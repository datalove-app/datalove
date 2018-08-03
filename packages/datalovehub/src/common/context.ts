import SchemaLink = require('apollo-link-schema');
import { ILock } from '../util/promise-lock';
import {
  IEntitiesMap,
  ITransactionManager,
} from '../activitystreams/entityManagers/base';

type ResolverContextFunction = SchemaLink.SchemaLink.ResolverContextFunction;
export interface IContextInput {
  TransactionManager: ITransactionManager,
}
export interface IContext extends IContextInput {
  operationID: string,
  exports: IEntitiesMap,
  exportLock: ILock,
}

export default function contextCreatorFactory(context: IContextInput): ResolverContextFunction {
  const { TransactionManager } = context;
  const createContext = (): IContext => Object.assign({}, context, {
    exports: TransactionManager.newExportsMap(),
    exportLock: TransactionManager.newExportsLock(),
    operationID: TransactionManager.newOperationID(),
  });

  return createContext;
}
