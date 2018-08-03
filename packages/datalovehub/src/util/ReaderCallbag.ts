import { Monad2 } from 'fp-ts/lib/Monad';
import { Reader } from 'fp-ts/lib/Reader';
import * as readerT from 'fp-ts/lib/ReaderT';
import { Callbag, callbag } from './Callbag';

const readerTCallbag = readerT.getReaderT(callbag);

export const URI = 'ReaderCallbag';

export type URI = typeof URI;

export class ReaderCallbag<E, A> {
  readonly _A!: A;
  readonly _L!: E;
  readonly _URI!: URI;
  constructor(readonly run: (e: E) => Callbag<A>) {}

  public map<B>(f: (a: A) => B): ReaderCallbag<E, B> {
    return new ReaderCallbag<E, B>(readerTCallbag.map(f, this.run));
  }

  public of<E, B>(b: B): ReaderCallbag<E, B> {
    return of(b);
  }

  public ap<B>(fab: ReaderCallbag<E, (a: A) => B>): ReaderCallbag<E, B> {
    return new ReaderCallbag<E, B>(readerTCallbag.ap(fab.run, this.run));
  }

  public chain<B>(f: (a: A) => ReaderCallbag<E, B>): ReaderCallbag<E, B> {
    return new ReaderCallbag<E, B>(readerTCallbag.chain(a => f(a).run, this.run));
  }
}

const map = <E, A, B>(fa: ReaderCallbag<E, A>, f: (a: A) => B): ReaderCallbag<E, B> => {
  return fa.map(f);
};

const of = <E, A>(a: A): ReaderCallbag<E, A> => {
  return new ReaderCallbag<E, A>(readerTCallbag.of(a));
};

const ap = <E, A, B>(fab: ReaderCallbag<E, (a: A) => B>, fa: ReaderCallbag<E, A>): ReaderCallbag<E, B> => {
  return fa.ap(fab);
};

const chain = <E, A, B>(fa: ReaderCallbag<E, A>, f: (a: A) => ReaderCallbag<E, B>): ReaderCallbag<E, B> => {
  return fa.chain(f);
};

export const readerTFromReader = readerT.fromReader(callbag);
export const fromReader = <E, A>(fa: Reader<E, A>): ReaderCallbag<E, A> => {
  return new ReaderCallbag<E, A>(readerTFromReader(fa));
};

export const readerCallbag: Monad2<URI> = {
  URI,
  map,
  of,
  ap,
  chain,
};
