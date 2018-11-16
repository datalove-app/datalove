# Multicodec

Elixir implementation of [Multicodec](https://github.com/multiformats/multicodec), a self-describing protocol codec.

> Compact self-describing codecs. Save space by using predefined multicodec tables.

## Motivation

From the official Multicodec README:
 
[Multistreams](https://github.com/multiformats/multistream) are self-describing protocol/encoding streams. Multicodec uses an agreed-upon "protocol table". It is designed for use in short strings, such as keys or identifiers (i.e [CID](https://github.com/ipld/cid)).
 
## Protocol Description - How does the protocol work?
 
`multicodec` is a _self-describing multiformat_, it wraps other formats with a tiny bit of self-description. A multicodec identifier may either be a varint (in a byte string) or a symbol (in a text string).

A chunk of data identified by multicodec will look like this:

```sh
<multicodec><encoded-data>
# To reduce the cognitive load, we sometimes might write the same line as:
<mc><data>
```

Another useful scenario is when using the multicodec as part of the keys to access data, example:

```
# suppose we have a value and a key to retrieve it
"<key>" -> <value>

# we can use multicodec with the key to know what codec the value is in
"<mc><key>" -> <value>
```
 
It is worth noting that multicodec works very well in conjunction with [multihash](https://github.com/multiformats/multihash) and [multiaddr](https://github.com/multiformats/multiaddr), as you can prefix those values with a multicodec to tell what they are.

## Usage

Typically you will have some data already in some format, then use Multicodec to explicitly add meta information on top of it. 

Note: For the sake of example, we will use simple Elixir binaries that may or may not match the encoding type to simplify things a bit here.

First, let's see what codecs have been loaded and are available for use:

```elixir
# many more returned than shown....
 Multicodec.codecs()
["raw", "cbor", "protobuf", "rlp", "bencode", "multicodec", "multihash",
 "multiaddr", "multibase", "identity", "md4", "md5", "sha1", "sha2-256",
 "sha2-512", "dbl-sha2-256", "sha3-224", "sha3-256", "sha3-384", "sha3-512",
 "shake-128", "shake-256", "keccak-224", "keccak-256", "keccak-384",
 "keccak-512", "murmur3", "x11", "blake2b-8", "blake2b-16", "blake2b-24",
 "blake2b-32", "blake2b-40", "blake2b-48", "blake2b-56", "blake2b-64",
 "blake2b-72", "blake2b-80", "blake2b-88", "blake2b-96", "blake2b-104",
 "blake2b-112", "blake2b-120", "blake2b-128", "blake2b-136", "blake2b-144",
 "blake2b-152", "blake2b-160", "blake2b-168", "blake2b-176", ...]
```

Let's encode some data. We take an elixir binary and supply the string name of the codec. The prefix associated with the given codec will be prepended to our data and is an unsigned varint.

```elixir
# some MD5 hashed data
 :crypto.hash(:md5, "you couldn't lead a monkey to a banana raffle") 
 |> Multicodec.encode!("md5")
<<213, 1, 110, 218, 175, 213, 125, 171, 92, 20, 46, 163, 55, 91, 118, 19, 14,
  109>>

# we can use a pattern matching friendly version too
Multicodec.encode("https://www.nhl.com/", "https")
{:ok,
<<187, 3, 104, 116, 116, 112, 115, 58, 47, 47, 119, 119, 119, 46, 110, 104,
 108, 46, 99, 111, 109, 47>>}

# What could go wrong with encoding? I'm glad you asked.
Multicodec.encode("astroturfed cookie recipe", "cookie-crime")
{:error, "unsupported codec - \"cookie-crime\""}
```

Let's decode our data. Notice you don't need to specify the codec because it is stored with the data.

```elixir
# decoding our https example from before
Multicodec.decode!(<<187, 3, 104, 116, 116, 112, 115, 58, 47, 47, 119, 119, 119, 46, 110, 104, 108, 46, 99, 111, 109, 47>>)
"https://www.nhl.com/"

# transparently encoding + decoding some bencoded torrent data, but with our error handling version now
 Multicodec.encode!("d3:fool3:bar3:baze3:qux4:norfe", "torrent-info") |> Multicodec.decode()
{:ok, "d3:fool3:bar3:baze3:qux4:norfe"}
```

That's great, but maybe we need to do further work on our data like decode the payload or perhaps validate it is what we expected.

One way to do this is to just return the codec alongside the decoded data. Fortunately, Multicodec makes this trivial using `codec_decode/1` and `codec_decode!/1`. You should prefer these if you need (as is common) the codec information alongside the data.

```elixir
# now we get back the codec as the second element 
# we can dynamically decode this using some bencode decoder lib if we wanted (not shown here)
Multicodec.encode!("d3:fool3:bar3:baze3:qux4:norfe", "torrent-info") |> Multicodec.codec_decode!()
{"d3:fool3:bar3:baze3:qux4:norfe", "torrent-info"}

# we can also encode this using identity, which just tags it still accordingly
# let's just do that and decode it using the error handling version now
 Multicodec.encode!("The grass is several shades of blue, Every member of parliament trips on glue", "identity") 
 |> Multicodec.codec_decode()
{:ok,
 {"The grass is several shades of blue, Every member of parliament trips on glue",
  "identity"}}
```

Suppose we only cared about what the codec is, perhaps for auditing of validation. No problem.

````elixir
encoded_data = Multicodec.encode!("2c4dc0085db090f0ec3f44c5285fab242f23e88f18fbfdfe72fb4d068b12aca5", "bitcoin-block")
# sent across a wire or elsewhere in the code perhaps, but let's assume we got it and just want to check the codec
encoded_data |> Multicodec.codec!()
"bitcoin-block"

# or using error handling and all at once
:crypto.hash(:sha256, "Do you like Phil Collins") 
|> Multicodec.encode!("sha2-256") 
|> Multicodec.codec()
{:ok, "sha2-256"} 
````

Just to be sure you understand what's really going on, let's take a quick look at prefixes again.

```elixir
# What's the prefix for blake2b-504?
Multicodec.prefix_for!("blake2b-504")
# notice we get multiple bytes
<<191, 228, 2>>

# prefixes can be a variety of byte sizes, depending on the code that is encoded as an unsigned varint
Multicodec.prefix_for("raw")
# notice it prints "U", but since we're Elixir savy, we know this is the same thing as <<85>> 
{:ok, "U"}

# We can take a full look at the list of mappings in the system if we're curious
Multicodec.mappings()
# many more, truncated for sanity
[
  %Multicodec.MulticodecMapping{code: 85, codec: "raw", prefix: "U"},
  %Multicodec.MulticodecMapping{code: 81, codec: "cbor", prefix: "Q"},
  %Multicodec.MulticodecMapping{code: 80, codec: "protobuf", prefix: "P"},
  %Multicodec.MulticodecMapping{code: 96, codec: "rlp", prefix: "`"},
  %Multicodec.MulticodecMapping{code: 99, codec: "bencode", prefix: "c"},
  %Multicodec.MulticodecMapping{code: 48, codec: "multicodec", prefix: "0"},
  %Multicodec.MulticodecMapping{code: 49, codec: "multihash", prefix: "1"},
  %Multicodec.MulticodecMapping{code: 50, codec: "multiaddr", prefix: "2"},
  %Multicodec.MulticodecMapping{code: 51, codec: "multibase", prefix: "3"},
  %Multicodec.MulticodecMapping{code: 0, codec: "identity", prefix: <<0>>},
  %Multicodec.MulticodecMapping{code: 212, codec: "md4", prefix: <<212, 1>>},
  %Multicodec.MulticodecMapping{code: 213, codec: "md5", prefix: <<213, 1>>},
  ...]

# and since it's just a list map, we can transform it however we want if we want some more info
# list all the codes as hex, for fun in the most inefficient way possible
 Multicodec.mappings() 
 |> Enum.map(fn(%{code: code}) -> Integer.to_string(code, 16) 
 |>  (&("0x" <> &1)).() end)
# truncated for sanity
["0x55", "0x51", "0x50", "0x60", "0x63", "0x30", "0x31", "0x32", "0x33", "0x0",
 "0xD4", "0xD5", "0x11", "0x12", "0x13", "0x56", "0x17", "0x16", "0x15", "0x14",
 "0x18", "0x19", "0x1A", "0x1B", "0x1C", "0x1D", "0x22", "0x1100", "0xB201",
 "0xB202", "0xB203", "0xB204", "0xB205", "0xB206", "0xB207", "0xB208", "0xB209",
 "0xB20A", "0xB20B", "0xB20C", "0xB20D", "0xB20E", "0xB20F", "0xB210", "0xB211",
 "0xB212", "0xB213", "0xB214", "0xB215", "0xB216", ...]
```

## New Codecs

Multicodec tables can grow over time. As such, this library dynamically generates the list of codecs and includes only those codecs that are official supported by the Multicodec standard.

The current approach is in flux, however it is relatively simple. The codec mappings (codes, prefixes, codecs) are first generated with a function call, and then added to a module-level variable in a moduleTo avoid any ambiguity, surprises, debugging, and potential security issues. There is support for directly loading and transforming the official [Multicodec table](https://github.com/multiformats/multicodec#multicodec-table) which is available as a [CSV](https://github.com/multiformats/multicodec/blob/master/table.csv) file.

As some decisions are still in flux such as [which codecs to support](https://github.com/multiformats/multicodec/issues/89), the manual approach seems more prudent.

The functions that deal with reading the official CSV are available in `Multicodec.CodecParser`. 

For example, the following calls may be of use:

```elixir
# prints out all parsed codecs, using the repo bundles CSV.
# perhaps write it to a file if you want 
Multicodec.CodecParser.parse_table()
# returns a giant list of maps

# optionally, you can pass a path
Multicodec.CodecParser.parse_table("directory_of_doom/table.csv")

# You can also print the codecs to stdout or a given device
Multicodec.CodecParser.inspect_table()
# prints giant list of maps

``` 

Relevant Modules:

* `Multicodec.CodecTable` - Full, generated codec table. Used by macros/fragments to emit code.
* `Multicodec.CodecParser` - Tools for processing the source data from the official CSV.

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `multicodec` to your list of dependencies in `mix.exs`:

## Installation

Mulicodec is available via [Hex](https://hex.pm/packages/multicodec). The package can be installed by adding `multicodec` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:multicodec, "~> 0.0.1"}
  ]
end
```

API Documentation can be found at [https://hexdocs.pm/multicodec/](https://hexdocs.pm/multicodec).

## Acknowledgements

* "Motivation" and "Protocol Description" sections are quoted from [Multicodec Official](https://github.com/multiformats/multicodec)
