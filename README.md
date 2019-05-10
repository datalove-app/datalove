# Elixir CBOR

An implementation of of [RFC 7049](https://tools.ietf.org/html/rfc7049) Concise Binary Object Representation (CBOR) for Elixir.

__This is alpha quality software. Only a small subset of types are supported.
Use at your own risk. [excbor](https://github.com/cabo/excbor) looks like it's
no longer maintained. If anyone else has a more fully feaured implementation of
CBOR in Elixir open an issue and I'll get the repo and package transferred.__
## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `cbor` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:cbor, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at [https://hexdocs.pm/cbor](https://hexdocs.pm/cbor).


## Usage

    iex(2)> bytes = Cbor.encode(%{array: [1,2,3], map: %{key: :value}, string: :string})
    <<163, 102, 115, 116, 114, 105, 110, 103, 102, 115, 116, 114, 105, 110, 103, 99, 109, 97, 112, 161, 99, 107, 101, 121, 101, 118, 97, 108, 117, 101, 101, 97, 114, 114, 97, 121, 131, 1, 2, 3>>
    iex(3)> Cbor.decode(bytes)
    %{array: [1, 2, 3], map: %{key: :value}, string: :string}
