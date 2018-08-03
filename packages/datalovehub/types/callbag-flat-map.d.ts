import { Callbag, CallbagOperator1 } from './callbag';

export type CallbagFlatMap<S, T> = CallbagOperator1<(s: S) => Callbag<T>, S, T>;
