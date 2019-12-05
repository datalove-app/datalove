// enum CompoundEncoder<'a, E, N>
// where
//     E: Encoder,
//     N: Node<'a>,
// {
//     List(N::ListIter, E::SerializeSeq),
//     Map(N::MapIter, E::SerializeMap),
// }

// impl<'a, E, N> CompoundEncoder<'a, E, N>
// where
//     E: Encoder,
//     N: Node<'a>,
// {
//     fn list_enc(&self) -> Option<E::SerializeSeq> {
//         match self {
//             CompoundEncoder::List(_, enc) => unsafe { Some(mem::transmute_copy(enc)) },
//             CompoundEncoder::Map(_, _) => None,
//         }
//     }

//     fn map_enc(&self) -> Option<E::SerializeMap> {
//         match self {
//             CompoundEncoder::Map(_, enc) => unsafe { Some(mem::transmute_copy(enc)) },
//             CompoundEncoder::List(_, _) => None,
//         }
//     }
// }

// struct NodeEncoderIterator<'a, E, N>
// where
//     // W: std::io::Write,
//     E: Encoder,
//     N: Node<'a>,
// {
//     // count?
//     root: &'a N,
//     encoder: E,
//     stack: Vec<CompoundEncoder<'a, E, N>>,
// }

// impl<'a, E, N> NodeEncoderIterator<'a, E, N>
// where
//     E: Encoder,
//     N: Node<'a>,
// {
//     fn _new(&mut self, node: &'a N) -> Self {
//         NodeEncoderIterator {
//             root: node,
//             encoder: self.copy_encoder(),
//             stack: unsafe { mem::transmute_copy(&self.stack) }
//         }
//     }

//     /*
//      *  - get type of node
//      *  - if not list or map
//      *      - call `node.serialize(encoder)`, return ...?
//      *  - if list (or map, same logic)
//      *      - create iter
//      *      - call `encoder.serialize_[seq/map](len)`, which creates a CompoundEncoder
//      *      - append (iter, compound_encoder) to stack, return ...?
//      */
//     fn encode(&mut self, node: &'a N) -> Result<(), E::Error> {
//         match node.kind() {
//             Token::List(len) => {
//                 let iter = node.list_iter().unwrap();
//                 let enc = self.encode_seq(len)?;
//                 self.stack.push(CompoundEncoder::List(iter, enc));
//                 Ok(())
//             }
//             Token::Map(len) => {
//                 let iter = node.map_iter().unwrap();
//                 let enc = self.encode_map(len)?;
//                 self.stack.push(CompoundEncoder::Map(iter, enc));
//                 Ok(())
//             }
//             _ => {
//                 node.serialize(self.copy_encoder())?;
//                 Ok(())
//             }
//         }
//     }

//     fn encode_seq(&mut self, len: Option<usize>) -> Result<E::SerializeSeq, E::Error> {
//         self.copy_encoder().serialize_seq(len)
//     }

//     fn encode_map(&mut self, len: Option<usize>) -> Result<E::SerializeMap, E::Error> {
//         self.copy_encoder().serialize_map(len)
//     }

//     /// Makes an unsafe copy of the owned `Encoder`.
//     fn copy_encoder(&self) -> E {
//         unsafe { mem::transmute_copy(&self.encoder) }
//     }
// }

// impl<'a, E, N> Serialize for NodeEncoderIterator<'a, E, N> where E: Encoder, N: Node<'a> {
//     fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
//         self.encode(self.root)
//     }
// }

// enum NodeEncoderIteratorResult {
//     Ok,
//     Incomplete,
// }

// impl<'a, E, N> Iterator for NodeEncoderIterator<'a, E, N>
// where
//     E: Encoder,
//     N: Node<'a>,
// {
//     type Item = ();

//     /*
//      *  - if stack is empty
//      *      - call `self.encode(self.root)`
//      *  - else
//      *      - grab last on stack
//      *      - call `last.0.next()` on the iter
//      *          - if elem
//      *              - create NodeEncoderIterator from node and self
//      *              - call `last.1.serialize_element(nodeEncoderIterator)`
//      *          - if key-value pair
//      *              - call `last.1.serialize_key(key)`
//      *              - create NodeEncoderIterator from value and self
//      *              - call `last.1.serialize_value(nodeEncoderIterator)`
//      *          - else if empty
//      *              - call `last.1.end()`
//      *
//      *
//      */
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.stack.len() == 0 {
//             return match self.encode(self.root) {
//                 Ok(()) => Some(()),
//                 Err(_) => None,
//             };
//         }

//         let compound_enc = self.stack.pop().unwrap();
//         match compound_enc {
//             CompoundEncoder::List(mut iter, enc) => match iter.next() {
//                 None => {
//                     if enc.end().is_err() {
//                         return None;
//                     }

//                     Some(())
//                 }
//                 Some(elem) => {
//                     let compound_enc = CompoundEncoder::List(iter, enc);
//                     let mut temp_enc = compound_enc.list_enc().unwrap();
//                     self.stack.push(compound_enc);

//                     // TODO: serialize element
//                     temp_enc.serialize_element(elem);

//                     Some(())
//                 }
//             },
//             CompoundEncoder::Map(mut iter, enc) => match iter.next() {
//                 None => {
//                     if enc.end().is_err() {
//                         return None;
//                     }

//                     Some(())
//                 }
//                 Some((key, value)) => {
//                     let compound_enc = CompoundEncoder::Map(iter, enc);
//                     let mut temp_enc = compound_enc.map_enc().unwrap();
//                     self.stack.push(compound_enc);

//                     // TODO: serialize key + value

//                     Some(())
//                 }
//             },
//         }

//         // if self.stack.len() == 0 {
//         //     self.stack.push()
//         // }
//     }
// }
