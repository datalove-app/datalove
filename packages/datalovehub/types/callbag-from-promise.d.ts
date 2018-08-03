import { SourceInitiator } from './callbag';

export type CallbagMapPromise<T> = (p: Promise<T>) => SourceInitiator<T>;
