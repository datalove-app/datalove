import lensPath from 'ramda/src/lensPath';
import set from 'ramda/src/set';
import {
  IEntityOperation,
  IEntityOperationInput,
  IEntityOperationCreator,
  IEntityTaskResolver,
} from './index';
import { varsFromGraphQLArgs } from './util';
import * as Context from '../../../common/context';
import * as FileURN from '../../../common/FileURN';
import {
  IEntityManager,
  IEntityManagerStatic,
} from '../../entityManagers/base';

export interface ILink {}
export interface ICreationOperationInputLink {
  fromLink: string,
  to: string,
}
export interface ICreationOperation<IArgs> {}
export interface ICreationOperationInput {
  id: string,
  fileName: string,
  input: string,
  options?: string,
  links?: ICreationOperationInputLink[],
  exportName?: string,
  return?: boolean,
}
export interface ICreationOperationCreator {
  <IArgs>(input: ICreationOperationInput): ICreationOperation<IArgs>,
}

const DEFAULT_RELEASE = () => Promise.resolve();

async function setLink(object: IEntityManager, linkProp: string, target: IEntityManager) {
  return object.update((state: any) => {
    const linkPath = linkProp.split('.');
    const lens = lensPath(linkPath);
    return set(lens, target.id(), state);
  });
}

async function setLinks(object: IEntityManager, links: ICreationOperationInputLink[], context: Context.IContext) {
  const { exports, TransactionManager } = context;
  const { entityLock } = TransactionManager;
  const linkUpdates = links.map(async ({ fromLink, to }) => {
    const releaseExport = await entityLock(to);
    const target = exports.get(to);

    if (typeof target === 'undefined') {
      throw new Error(`Entity \`${to}\` is undefined`);
    }

    try {
      await setLink(object, fromLink, target);
    } catch (err) {
      throw new Error(`Cannot update entity \`${object.id()}\`: ${err.message}`);
    } finally {
      await releaseExport();
    }
  });

  return Promise.all(linkUpdates);
}

export default function makeOperationCreator<IArgs>(EntityManager: IEntityManagerStatic): ICreationOperationCreator {
  return function createOperation<IArgs>(entityResolverArgs: ICreationOperationInput): ICreationOperation<IArgs> {
    const {
      id: idArg, fileName: nameArg, input, options: optionsArg,
      links = [], exportName, return: shouldReturn = false,
    } = entityResolverArgs;
    const hasExportName = typeof exportName === 'string';

    const resolver: IEntityTaskResolver<IArgs> = async (_root, args, context) => {
      const { exports, exportLock, TransactionManager } = context;
      const { entities, entityLock } = TransactionManager;
      const [fileURN, fileName, inputState, options] = varsFromGraphQLArgs(
        [idArg, nameArg, input, optionsArg] as string[],
        args
      );
      const id = fileURN
        || FileURN.fromDefaults({ fileName }, TransactionManager)
        || FileURN.new({ fileName }, TransactionManager);

      let entityRelease = DEFAULT_RELEASE;
      let exportRelease = DEFAULT_RELEASE;
      let object: IEntityManager;

      try {
        entityRelease = await entityLock(id);
        exportRelease = hasExportName
          ? await exportLock(exportName as string)
          : DEFAULT_RELEASE;

        object = entities.get(id)
          || await EntityManager.create(id);
        await setLinks(object, links, context);

        entities.set(id, object);
        exports.set(exportName as string, object);
        await exportRelease();
      } catch (err) {
        await entityRelease();
        await exportRelease();
        throw new Error(`Failed to create object with id \`${id}\` and exportName \`${exportName}\`: ${err.message}`);
      }

      const task = {
        name: object.id(),
        perform: async () => {
          await object.commit();
          return entityRelease();
        },
        rollback: async () => {
          await object.rollback();
          return entityRelease();
        },
      };

      return shouldReturn
        ? { task, finalResolver: () => object.view() }
        : { task };
    };

    // const resolver: IEntityResolver = async (_root, args, context) => {
    //   const { exports, TransactionManager } = context;
    //   const { blockstack, entityLock } = TransactionManager;
    //   const [name, inputState, options] =
    //     varsFromGraphQLArgs([nameArg, input, optionsArg], args);
    //
    //   let releaseExport = DEFAULT_RELEASE;
    //   if (hasExportName) {
    //     releaseExport = await entityLock(exportName as string);
    //   }
    //
    //   const objectName = exportName || name;
    //   let object: IEntityManager;
    //
    //   let err = null;
    //   try {
    //     object = await EntityManager
    //       .from(name, inputState, blockstack, options);
    //     if (links.length > 0) {
    //       await setLinks(object, objectName, links, context);
    //     }
    //
    //     if (hasExportName) {
    //       exports.set(exportName as string, object);
    //     }
    //   } catch (_err) {
    //     err = _err;
    //   } finally {
    //     await releaseExport();
    //   }
    //
    //   if (err !== null) {
    //     throw err;
    //   }
    //   return { object, commit: () => object.commit() };
    // };

    return exportName
      ? { exportName, resolver, shouldReturn }
      : { resolver, shouldReturn };
  };
}
