import {
  GraphQLResolveInfo,
  GraphQLFieldResolver,
} from 'graphql';
import Transaction from 'promise-transaction';
import { IEntityOperation } from '../mutationOperations/base';
import Context = require('../../common/context');

export type IEntityOperations<IArgs> = IEntityOperation<IArgs>[];
export type IMutationResolver<IArgs> = GraphQLFieldResolver<any, Context.IContext, IArgs>;

const DEFAULT_RESOLVE = () => Promise.resolve(null);
const DEFAULT_ROLLBACK = () => Promise.resolve(false);
const DEFAULT_FINAL_TASK = {
  name: 'finalResolver',
  perform: DEFAULT_RESOLVE,
  rollback: DEFAULT_ROLLBACK,
};

export const createQueryResolver = () => {
  const queryResolver = async (root, args, context, info) => {
    return null;
  };

  return queryResolver;
};

export const createMutationOperationResolver = <IArgs>(operations: IEntityOperations<IArgs>): IMutationResolver<IArgs> => {
  /**
   * each operation returns an IEntityResolver
   * when invoked, it returns a promise to an IEntityManager of the in-memory
   *  object that was created/updated
   */

  const mutationResolver: IMutationResolver<IArgs> = async (root, args, context, info) => {
    /**
     * create exports map
     * create transaction array
     * for each operation:
     *  - await the operation, giving it all args and exports
     *    - if required, attach returned object to export map
     *  - create a commit function and tx task, append it to transaction array
     * create transaction
     * return processed transaction
     */
    let finalTask = DEFAULT_FINAL_TASK;
    const commitTasks = await Promise.all(
      operations
        .map(async function createCommit({ resolver }) {
          const { task, finalResolver } = await resolver(root, args, context, info);
          if (finalResolver) {
            finalTask = {
              name: 'finalResolver',
              perform: finalResolver,
              rollback: DEFAULT_ROLLBACK,
            };
          }
          return task;
        })
    );

    const transaction = new Transaction(commitTasks.concat(finalTask));
    return transaction.process();
  };

  return mutationResolver;
};
