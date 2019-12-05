//! General Notes:
//!     - Tokens should borrow everything, even primitives
//!         - reimpl using CowT
//!     - new decoder
//!         - [format] write a parser == <&[u8], Token> || <&str, Token>
//!         - write ...... == selector == `select(&selector)`
//!             -
//!             - enqueues x, matches, enqueues y
//!
//!     - read/write patterns (look at css-parser and other servo projects)
//!         - sub-select dag(s) from a block
//!         - sub-select dag(s) from multiple blocks (by traversing links)
//!         | update-within block
//!
//!     - mod selector (look at updated selector spec)
//!         - parser:
//!             -
//!         - types:
//!              - SelectorEnvelope
//!              - Selector enum of SelectorTypes
//!              - SelectorType (may contain selectors)
//!              - ? SelectionResult ?
//!
//! Pipelines (mixing elixir and rust):
//!     - single-block, single-chunk pipeline:
//!     - single-block pipeline:
//!         block (@> block chunks)
//!         @> codec parser
//!             (block + tokens)
//!         @> path/selector parser combinator over tokens (reader?)
//!             (block + cownode (+ selector?))
//!
//!         @> path/selector parser combinator over cownode (writer?)
//!             (block + new_cownode)
//!         @> ?serializer
//!             (yield by token)
//!         block
//!
//!     - multi-block pipeline:
//!         root block (@>block chunks)
//!         @> codec parser
//!             (block + tokens)
//!         @> path/selector parser combinator over tokens (reader?)
//!             (multi-block + cownode (+ selector?))
//!
//!         @> path/selector parser combinator over cownode (writer?)
//!             (multi-block + cownode)
//!         @> ?serializer
//!             (yield by token... and original/containing blocks?)
//!         multi-block
//! Potential Improvements:
//!     - cownode is best when performing quick updates:
//!         ?? performing one-off mutations (b/c it must hold onto all data)
//!     - freenode is best when manipulating "live" state:
//!         ?? multi-block selectors?
//!     - [SPECIAL] ** encoding a mutated dag **
//!         begin by skipping block chunks until reaching node's start index
//!         starts encoding node, which starts writing to output binary/string
//!         when encountering a link
//!             - yield the original CID of the linked node
//!             - pump in original blocks bytes
//!             - queues encoding nested node, embeds resulting CID
//!             - yields the string and the CID
//!             - continues encoding node
//!         finishes by yielding any remaining/pumped bytes




//! Breakdown of pipeline steps and APIs
//!     - block - Block::new() or Block::from_chunk()
//!         + ?? Block::extend()
//!     -
//!
//! Example API (from scraper):
/*
use scraper::{Html, Selector};
let html = r#"
    <ul>
        <li>Foo</li>
        <li>Bar</li>
        <li>Baz</li>
    </ul>
"#;

let ul_selector = Selector::parse("ul").unwrap();
let li_selector = Selector::parse("li").unwrap();
let fragment = Html::parse_fragment(html);

let ul = fragment.select(&ul_selector).next().unwrap();
for element in ul.select(&li_selector) {
    assert_eq!("li", element.value().name());
}
*/
//! Example API (adapted):
//!     -
/*
let raw_selector = b"..."
let first_chunk = b"...";

let selector = Selector::new(&raw_selector).unwrap();
let block = Block::new(CID, &first_chunk);

let selection = block.select(&tokenizer, &selector);


*/
//! Example API (with elixir in mind):
//!     `new(binary, \\ [])`
//!         - passes a binary and args, returns a resource struct
//!     `yield(resource, \\ [])`
//!         - forces computation,
//!         - returns tagged tuple or {:yield, args} | {:pump, args}
//!     `pump(resource, binary)`
//!         -
/*
type
type SelectionResult =
    | {:error, reason}
    | {:yield, BlockResource}
    | {:continue, {Node*, CID}}
    | {:ok, {Node*}}

fn new (chunk) -> Resource {
    Block::new(CID, first_chunk);
}

fn drive(resource) -> Result

fn steer(resource, chunk) -> Result

////////////////////////////////////////
let raw_selector = b"..."
let first_chunk = b"...";

let selector = Selector::parse(&raw_selector).unwrap();
let selection_iter = block.select(&selector);

*/
