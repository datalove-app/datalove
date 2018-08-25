type Omit<T, K extends keyof T> = Pick<T, Exclude<keyof T, K>>;

/**
 * Wrapper around `property` (to ensure returned values are native JS rather than just strings)
 * @param {string} propName
 * @returns {any}
 */
export const getProperty = (propName: string) => JSON.parse(property(propName));

/**
 * Same as `get`, but automatically specifies the `Local: true` option
 * @param {Hash} hash
 * @param {Omit<IGetOptions, "Local">} options
 * @returns {HashNotFound | TReturn}
 */
export function getLocal<TReturn>(hash: Hash, options?: Omit<IGetOptions, 'Local'>): HashNotFound | TReturn {
  return typeof options === 'undefined'
    ? get<TReturn>(hash, { Local: true })
    : get<TReturn>(hash, { ...options, Local: true });
}
