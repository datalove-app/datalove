import enums from './enums';
import scalars, { resolvers as scalarResolvers } from './scalars/index';
import unions from './unions/index';
import core from './core/index';
import { mergeSchemas } from './utils';

export { scalarResolvers };
export default mergeSchemas([enums, scalars, unions, core]);
