declare module 'lru-cache' {
  export interface Options {
    dispose?:(key: any, value: any) => void,
    length?: (value: any, key: any) => number,
    max?: number,
    maxAge?: number,
    noDisposeOnSet?: boolean,
    stale?: boolean,
  }

  export interface ILRU {
    length: number,
    itemCount: number,
    get: (key: any) => any,
    set: (key: any, value: any, maxAge: number) => void,
    del: (key: any) => void,
    reset: () => void,
    has: (key: string) => boolean,
    peek: (key: any) => any | void,
    keys: () => any[],
    values: () => any[],
    dump: () => void,
    prune: () => void,
  }

  export function LRU(optionsOrMaxAge: Options | number): ILRU;
}
