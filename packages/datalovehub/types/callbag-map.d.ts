import { CallbagOperator1 } from './callbag';

export type CallbagMap<S, T> = CallbagOperator1<(s: S) => T, S, T>;
