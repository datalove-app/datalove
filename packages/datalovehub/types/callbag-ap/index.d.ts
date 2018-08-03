import { Callbag, SinkTalkbackConnector } from '../callbag';

declare function ap<S, T>(m: Callbag<(s: S) => T>): SinkTalkbackConnector<S, T>;

export = ap;
