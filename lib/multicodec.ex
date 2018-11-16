defmodule Multicodec do
  @moduledoc """
  This module provides encoding, decoding, and convenience functions for working with [Multicodec](https://github.com/multiformats/multicodec).

  ## Overview

  > Compact self-describing codecs. Save space by using predefined multicodec tables.

  ## Motivation

   [Multistreams](https://github.com/multiformats/multistream) are self-describing protocol/encoding streams. Multicodec uses an agreed-upon "protocol table". It is designed for use in short strings, such as keys or identifiers (i.e [CID](https://github.com/ipld/cid)).

  ## Protocol Description

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

  ## Codecs

  All codecs are passed as strings. The reason for this is to avoid burdening the consumer with an ever-growing list of atoms that can contribute to exhausting the atom pool of a VM.

  If you would like to translate these strings into atoms, they are available via the `codecs/1` function, and can be transformed like so:

  ```elixir
  # probably you would want to fix the kebab casing too, not shown here
  Multicodec.codecs() |> Enum.map(&String.to_atom/1)
  ```

  Codecs can only be added if they are added officially to Multicodec. We do not deviate from the standard when possible.

  ## Encoding

  All data to encode should be an Elixir binary. It is up to the caller to properly encode the given payload, and it is the job of Multicodec to add metadata to describe that payload. Encoding using Multicodec does not perform any extra encoding or transformation on your data. It simply adds an unsigned variable integer prefix to allow unlimited amounts of codecs, and to cleanly decode them, returning the codec if desired.

  # Decoding

  There are 2 main ways of decoding data. The first and most common is to use `codec_decode/1` and `codec_decode!/1`, which return a tuple of `{data, codec}`. The second option if you do not care about the codec in some piece of code is to use `decode/1` and `decode!/1`. Multicodec does not modify the returned data - it is up to you if you need further decoding, for example decoding `bencode`.

  """

  alias Multicodec.{MulticodecMapping, CodecTable}

  @typedoc """
  A binary encoded with Multicodec.
  """
  @type multicodec_binary() :: binary()

  @typedoc """
  A codec used to encode a binary as a Multicodec.
  """
  @type multi_codec() :: MulticodecMapping.multi_codec()

  @typedoc """
  A binary representation of a multicodec code encoded as an unsigned varint.
  """
  @type prefix() :: MulticodecMapping.prefix()


  @doc """
  Encodes a binary using Multicodec using the given codec name.

  Raises an ArgumentError if the codec does not exist or the provided data is not a binary.

  ## Examples

      iex> Multicodec.encode!("d3:fool3:bar3:baze3:qux4:norfe", "bencode")
      "cd3:fool3:bar3:baze3:qux4:norfe"

      iex> Multicodec.encode!(<<22, 68, 139, 191, 190, 36, 62, 35, 171, 224, 129, 249, 63, 46, 47, 7, 119, 7, 178, 223, 184, 3, 249, 238, 66, 166, 153, 175, 101, 42, 40, 29>>, "sha2-256")
      <<18, 22, 68, 139, 191, 190, 36, 62, 35, 171, 224, 129, 249, 63, 46, 47, 7, 119, 7, 178, 223, 184, 3, 249, 238, 66, 166, 153, 175, 101, 42, 40, 29>>

      iex> Multicodec.encode!("legal_thing.torrent", "torrent-file")
      "|legal_thing.torrent"

  """
  @spec encode!(binary(), multi_codec()) :: multicodec_binary()
  def encode!(data, codec) when is_binary(data) and is_binary(codec) do
    <<do_prefix_for(codec)::binary, data::binary>>
  end

  def encode!(_data, _codec) do
    raise ArgumentError, "Data must be a binary and codec must be a valid codec string."
  end

  @doc """
    Encodes a binary using Multicodec using the given codec name.

    Raises an ArgumentError if the codec does not exist or the provided data is not a binary.

    ## Examples

        iex> Multicodec.encode("EiC5TSe5k00", "protobuf")
        {:ok, "PEiC5TSe5k00"}

        iex> :crypto.hash(:sha, "secret recipe") |> Multicodec.encode("sha1")
        {:ok,
        <<17, 139, 95, 199, 243, 128, 172, 237, 254, 18, 189, 127, 227, 208, 152, 232,
         107, 238, 26, 35, 106>>}

        iex> Multicodec.encode("Taco Tuesday", "mr-yotsuya-at-ikkoku")
        {:error, "unsupported codec - \\"mr-yotsuya-at-ikkoku\\""}

  """
  @spec encode(binary(), multi_codec()) :: {:ok, multicodec_binary()} | {:error, term()}
  def encode(data, codec) do
    {:ok, encode!(data, codec)}
    rescue
      e in ArgumentError -> {:error, Exception.message(e)}
  end

  @doc """
  Decodes a Multicodec encoded binary.

  If you need the codec returned with the data, use `codec_decode!/1` instead.

  Raises an ArgumentError if the given binary is not Multicodec encoded.

  ## Examples

      iex> Multicodec.decode!(<<0, 99, 111, 117, 110, 116, 32, 98, 114, 111, 99, 99, 117, 108, 97>>)
      "count broccula"

      iex> Multicodec.decode!(<<51, 0, 99, 114, 105, 115, 112, 121>>)
      <<0, 99, 114, 105, 115, 112, 121>>

      iex> :crypto.hash(:md5, "soup of the eon") |> Multicodec.encode!("md5") |> Multicodec.decode!()
      <<83, 202, 110, 26, 47, 119, 193, 71, 113, 201, 88, 92, 162, 222, 37, 108>>

  """
  @spec decode!(multicodec_binary()) :: binary()
  def decode!(data) when is_binary(data) do
    do_decode(data)
  end

  def decode!(_data) do
    raise ArgumentError, "data must be a Multicodec encoded binary."
  end

  @doc """
  Decodes a Multicodec encoded binary.

  If you need the codec returned with the data, use `codec_decode/1` instead.

  Returns an error if the given binary is not Multicodec encoded.

  ## Examples

      iex> Multicodec.decode(<<0, 66, 101, 115, 116, 32, 76, 117, 115, 104, 32, 97, 108, 98, 117, 109, 44, 32, 83, 112, 111, 111, 107, 121, 32, 111, 114, 32, 83, 112, 108, 105, 116>>)
      {:ok, "Best Lush album, Spooky or Split"}

      iex> Multicodec.decode(<<224, 3, 104, 116, 116, 112, 58, 47, 47, 122, 111, 109, 98, 111, 46, 99, 111, 109>>)
      {:ok, "http://zombo.com"}


      iex> :crypto.hash(:md4, "pass@word") |> Multicodec.encode!("md4") |> Multicodec.decode()
      {:ok,
        <<110, 141, 9, 114, 67, 195, 143, 146, 109, 201, 188, 52, 200, 125, 93, 225>>}

      iex> Multicodec.decode(<<>>)
      {:error, "data is not Multicodec encoded."}

  """
  @spec decode(multicodec_binary()) :: {:ok, binary()} | {:error, term()}
  def decode(data) when is_binary(data) do
    {:ok, decode!(data)}
    rescue
      e in ArgumentError -> {:error, Exception.message(e)}
  end

  @doc """
  Decodes a Multicodec encoded binary, and returning a tuple of the data and the codec used to encode it.

  Raises an ArgumentError if the given binary is not Multicodec encoded.

  ## Examples

      iex> Multicodec.codec_decode!(<<0, 87, 104, 101, 110, 32, 116, 104, 101, 32, 112, 101, 110, 100, 117, 108, 117, 109, 32, 115, 119, 105, 110, 103, 115, 44, 32, 105, 116, 32, 99, 117, 116, 115>>)
      {"When the pendulum swings, it cuts", "identity"}

      iex> Multicodec.codec_decode!(<<51, 0, 99, 114, 105, 115, 112, 121>>)
      {<<0, 99, 114, 105, 115, 112, 121>>, "multibase"}

      iex> :crypto.hash(:md5, "soup of the eon") |> Multicodec.encode!("md5") |> Multicodec.codec_decode!()
      {<<83, 202, 110, 26, 47, 119, 193, 71, 113, 201, 88, 92, 162, 222, 37, 108>>, "md5"}

  """
  @spec codec_decode!(multicodec_binary()) :: {binary(), multi_codec()}
  def codec_decode!(data) when is_binary(data) do
    do_codec_decode(data)
  end

  def codec_decode!(_data) do
    raise ArgumentError, "data must be a Multicodec encoded binary."
  end

  @doc """
  Decodes a Multicodec encoded binary, and returning a tuple of the data and the codec used to encode it.

  Returns an error if the given binary is not Multicodec encoded.

  ## Examples

      iex> Multicodec.codec_decode(<<0, 83, 108, 111, 119, 100, 105, 118, 101, 32, 116, 111, 32, 109, 121, 32, 100, 114, 101, 97, 109, 115>>)
      {:ok, {"Slowdive to my dreams", "identity"}}

      iex> Multicodec.codec_decode(<<51, 0, 99, 114, 105, 115, 112, 121>>)
      {:ok, {<<0, 99, 114, 105, 115, 112, 121>>, "multibase"}}

      iex> Multicodec.codec_decode(<<>>)
      {:error, "data is not Multicodec encoded."}

  """
  @spec codec_decode(multicodec_binary()) :: {:ok,{binary(), multi_codec()}} | {:error, term()}
  def codec_decode(data) when is_binary(data) do
    {:ok, codec_decode!(data)}
    rescue
    e in ArgumentError -> {:error, Exception.message(e)}
  end

  @doc """
  Returns the codec used to encode a Multicodec encoded binary.

  Raises an ArgumentError if the given binary is not Multicodec encoded.

  ## Examples

      iex> Multicodec.codec!(<<0, 67, 105, 114, 99, 108, 101, 32, 116, 104, 101, 32, 111, 110, 101, 115, 32, 116, 104, 97, 116, 32, 99, 111, 109, 101, 32, 97, 108, 105, 118, 101>>)
      "identity"

      iex> :crypto.hash(:sha512, "F") |> Multicodec.encode!("sha2-512") |> Multicodec.codec!()
      "sha2-512"

      iex> Multicodec.codec!("q")
      "dag-cbor"

  """
  @spec codec!(multicodec_binary()) :: multi_codec()
  def codec!(data) when is_binary(data) do
    {_, codec} = codec_decode!(data)
    codec
  end

  @doc """
  Returns the codec used to encode a Multicodec encoded binary.

  Returns an error if the given binary is not Multicodec encoded.

  ## Examples

      iex> Multicodec.codec(<<6, 73, 32, 97, 109, 32, 97, 32, 115, 99, 105, 101, 110, 116, 105, 115, 116>>)
      {:ok, "tcp"}

      iex> Multicodec.codec(<<0x22>>)
      {:ok, "murmur3"}

      iex> Multicodec.encode!("I am a scientist, I seek to understand me", "identity") |> Multicodec.codec()
      {:ok, "identity"}

      iex> Multicodec.codec(<<>>)
      {:error, "data is not Multicodec encoded."}

  """
  @spec codec(multicodec_binary()) :: {:ok, multi_codec()} | {:error, term()}
  def codec(data) when is_binary(data) do
    {:ok, codec!(data)}
    rescue
      e in ArgumentError -> {:error, Exception.message(e)}
  end

  @doc """
  Returns a list of codecs that can be used to encode data with Multicodec.
  """
  @spec codecs() :: [multi_codec()]
  def codecs() do
    unquote(Enum.map(CodecTable.codec_mappings(), fn(%{codec: codec}) -> codec end))
  end

  @doc """
  Returns a full mapping of codecs, codes, and prefixes used by Multicodec.

  Each entry in the list is a mapping specification of how to encode data with Multicodec.
  """
  @spec mappings() :: [MulticodecMapping.t()]
  def mappings() do
    unquote(Macro.escape(CodecTable.codec_mappings))
  end

  @doc """
  Returns the prefix that should be used with the given codec.

  Raises an error if the given binary is not Multicodec encoded.

  ## Examples

      iex> Multicodec.prefix_for!("git-raw")
      "x"

      iex> Multicodec.prefix_for!("bitcoin-block")
      <<176, 1>>

      iex> Multicodec.prefix_for!("skein1024-512")
      <<160, 231, 2>>

  """
  @spec prefix_for!(multi_codec()) :: prefix()
  def prefix_for!(codec) when is_binary(codec) do
    do_prefix_for(codec)
  end


  @doc """
  Returns the prefix that should be used with the given codec.

  Returns an error if the given binary is not Multicodec encoded.

  ## Examples

      iex> Multicodec.prefix_for("blake2b-272")
      {:ok, <<162, 228, 2>>}

      iex> Multicodec.prefix_for("bitcoin-block")
      {:ok, <<176, 1>>}

      iex> Multicodec.prefix_for("ip6")
      {:ok, ")"}

      iex> Multicodec.prefix_for("Glorious Leader")
      {:error, "unsupported codec - \\"Glorious Leader\\""}


  """
  @spec prefix_for(multi_codec()) :: {:ok, prefix()} | {:error, term()}
  def prefix_for(codec) do
    {:ok, prefix_for!(codec)}
    rescue
      e in ArgumentError -> {:error, Exception.message(e)}
  end

#===============================================================================
# Private
#===============================================================================

  defp do_codec_decode(<<>>) do
    raise ArgumentError, "data is not Multicodec encoded."
  end

  defp do_codec_decode(data) do
    {prefix, decoded_data} = decode_varint(data) #Varint.LEB128.decode(data)
    {decoded_data, codec_for(prefix)}
  end

  defp do_prefix_for(codec)
  for %{prefix: prefix, codec: codec} <- CodecTable.codec_mappings() do
    defp do_prefix_for(unquote(codec)) do
      unquote(prefix)
    end
  end

  defp do_prefix_for(codec) when is_binary(codec) do
    raise ArgumentError, "unsupported codec - #{inspect codec, binaries: :as_strings}"
  end

  defp codec_for(code)
  for %{codec: codec, code: code} <- CodecTable.codec_mappings() do
    defp codec_for(unquote(code)) do
      unquote(codec)
    end
  end

  defp codec_for(code) when is_integer(code) do
    raise ArgumentError, "unsupported code - #{inspect code, binaries: :as_strings}"
  end

  defp do_decode(<<>>) do
    raise ArgumentError, "data is not Multicodec encoded."
  end

  defp do_decode(data) do
    {_prefix, decoded_data} = decode_varint(data) #Varint.LEB128.decode(data)
    decoded_data
  end

  defp decode_varint(data) do
    #temporary patch until we can replace or pull request varint
    Varint.LEB128.decode(data)
    rescue
      FunctionClauseError -> raise ArgumentError, "data is not a varint."
  end

end
