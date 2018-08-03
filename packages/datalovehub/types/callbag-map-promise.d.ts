import { CallbagOperator1 } from './callbag';

export type CallbagMapPromise<S, T> = CallbagOperator1<(s: S) => Promise<T>, S, T>;
