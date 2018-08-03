const SEPARATOR = '.';
const VAR_PREFIX = '$';

const isGraphQLVar = (name: string): boolean =>
  name.startsWith(VAR_PREFIX);

const removeVarPrefix = (varName: string): string =>
  varName.slice(1);

const varFromGraphQLArgs = (varName: string, graphQLArgs: {}): any => {
  const argName = removeVarPrefix(varName);
  const arg = graphQLArgs[argName];
  return (typeof arg !== 'undefined')
    ? arg
    : null;
};

export function varsFromGraphQLArgs(paths: string[], graphQLArgs: {}): any[] {
  return paths.map((path) => {
    if (typeof path !== 'string') return null;

    const parts = path.split(SEPARATOR);
    const varName = parts[0];
    if (!isGraphQLVar(varName)) return null;

    return parts.reduce((acc, subPath, index) => {
      if (acc === null) return null;
      if (index === 0) return varFromGraphQLArgs(subPath, acc);
      if (isGraphQLVar(subPath)) return varFromGraphQLArgs(subPath, acc);

      const arg = acc[subPath];
      return (typeof arg !== 'undefined')
        ? arg
        : null;
    }, graphQLArgs);
  });
};
