export = callbag;

declare namespace callbag {
  export type START = 0;
  export type DATA = 1;
  export type END = 2;

  export type SourceTalkback =
    & ((request: DATA) => void)
    & ((terminate: END) => void);
  export type SinkTalkback<T> =
    & ((start: START, sourceTalkback: SourceTalkback) => void)
    & ((deliver: DATA, data: T) => void)
    & ((terminate: END, error?: any) => void);

  export type SourceInitiator<T> = (start: START, sinkTalkback: SinkTalkback<T>) => void;
  export type SinkTalkbackConnector<S, T> = (source: SourceInitiator<S>) => SourceInitiator<T> | void;

  export type CallbagOperator1<A1, S, T> = (a1: A1) => SinkTalkbackConnector<S, T>;
  export type CallbagOperator2<A1, A2, S, T> = (a1: A1, a2: A2) => SinkTalkbackConnector<S, T>;
  export type CallbagOperator3<A1, A2, A3, S, T> = (a1: A1, a2: A2, a3: A3) => SinkTalkbackConnector<S, T>;
  export type CallbagFactory1<A1, T> = (a1: A1) => SourceInitiator<T>;
  export type CallbagFactory2<A1, A2, T> = (a1: A1, a2: A2) => SourceInitiator<T>;
  export type CallbagFactory3<A1, A2, A3, T> = (a1: A1, a2: A2, a3: A3) => SourceInitiator<T>;

  export type Callbag<T> =
    | SourceTalkback
    | SinkTalkback<T>
    | SourceInitiator<T>;
}
