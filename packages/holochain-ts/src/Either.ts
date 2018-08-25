export function isError<T extends any = any>(result: Either<T, IError>): result is IError {
  return typeof result === 'object' &&
    Object.prototype.hasOwnProperty.call(result, 'name') &&
    (result as IError).name === 'HolochainError';
}

export function isSuccess<T extends any = any>(result: Either<T, IError>): result is T {
  return !isError<T>(result);
}
