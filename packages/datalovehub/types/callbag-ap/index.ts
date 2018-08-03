import flatMap from 'callbag-flat-map';
import map from 'callbag-map';
import pipe from 'callbag-pipe';

import { Callbag } from '../callbag';
import Ap = require('../callbag-ap');

const ap: Ap<S, T> = (m: Callbag<(s: S) => T>) => (source: Callbag<S>) => pipe(
  source,
  flatMap((e: S) => pipe(
    m,
    map((fn: (s: S) => T) => fn(e))
  ))
);

export default ap;
