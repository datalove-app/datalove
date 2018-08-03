import cflatMap from 'callbag-flat-map';
import cmap from 'callbag-map';
import cof from 'callbag-of'
import pipe from 'callbag-pipe';
import { Monad1 } from 'fp-ts/lib/Monad';

import cap from '../../types/callbag-ap';
import { Callbag } from '../../types/Callbag';

export const URI = 'Callbag';

export type URI = typeof URI;

const map = <A, B>(fa: Callbag<A>, f: (a: A) => B): Callbag<B> => {
  return pipe(fa, cmap(f));
};

const of = <A>(a: A): Callbag<A> => {
  return cof(a);
};

const ap = <A, B>(fab: Callbag<(a: A) => B>, fa: Callbag<A>): Callbag<B> => {
  return pipe(fa, cap(fab))
};

const chain = <A, B>(fa: Callbag<A>, f: (a: A) => Callbag<B>): Callbag<B> => {
  return pipe(fa, cflatMap(f));
};

export const callbag: Monad1<URI> = {
  URI,
  map,
  of,
  ap,
  chain,
};
