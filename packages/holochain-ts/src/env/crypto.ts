/**
 * Makes a hash of the given entry data.
 * @param {string} entryType
 * @param {string | ILinksEntry} entryData
 * @returns {TReturn} TReturn extends (Hash | Either<Hash>) = Either<Hash, IError>
 */
declare function makeHash<TReturn extends (Hash | Either<Hash>) = Either<Hash>>(entryType: string, entryData: string | ILinksEntry | any): TReturn;

/**
 * Uses the agent's private key to sign a string.
 * @param {string} doc
 * @returns {TReturn} TReturn extends (string | Either<string>) = Either<string>
 */
declare function sign<TReturn extends (string | Either<string>) = Either<string>>(doc: string): TReturn;

/**
 * Uses the signature, data and signatory's public key to verify the signature of the contents of `data`. Result represents whether its a match or not.
 * @param {string} signature
 * @param {string} data
 * @param {string} publicKey
 * @returns {TReturn} TReturn extends (boolean | Either<boolean>) = Either<boolean>
 */
declare function verifySignature<TReturn extends (boolean | Either<boolean>) = Either<boolean>>(signature: string, data: string, publicKey: string): TReturn;
