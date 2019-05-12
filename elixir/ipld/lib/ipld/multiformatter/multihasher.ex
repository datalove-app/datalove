defmodule IPLD.Multiformatter.Multihasher do
  @moduledoc """
  Module for abstracting hashing over some of the various [Multihash](https://github.com/multiformats/multihash) algorithms.

  Uses [:ex_multihash](https://github.com/multiformats/ex_multihash) and borrows from [:multihashing](https://github.com/candeira/ex_multihashing).
  """

  alias Multihash
  alias IPLD.Format
  import IPLD.Multiformatter.Util

  @typedoc "Struct for describing a multihash, which can be serialized to binary."
  @type t :: %__MODULE__{
          codec: __MODULE__.multihash_codec(),
          length: non_neg_integer,
          digest: binary
        }
  @type multihash_codec :: Multihash.hash_type()
  @typedoc "The maximum desired length of a multihash digest. `:default` defers to the underlying hash algorithm's digest length."
  @type trunc_length :: :default | non_neg_integer

  @type on_from :: {:ok, Multihasher.t()} | {:error, reason :: atom}
  @type on_to :: {:ok, Format.blob()} | {:error, reason :: atom}
  @type on_verify :: {:ok, bool} | {:error, reason :: atom}

  @enforce_keys [:codec, :length, :digest]
  defstruct codec: @default_mh,
            length: 0,
            digest: <<>>

  @default_mh :sha2_256
  @default_len :default

  @typedoc "Tracks a mapping from multihash codecs to hash codes used by `:crypto.hash/2`, as well as their default digest lengths."
  @native_algo_map %{
    :md4 => {:md4, 16},
    :md5 => {:md5, 16},
    :sha1 => {:sha, 20},
    :sha2_256 => {:sha256, 32},
    :sha2_512 => {:sha512, 64},
    :sha3_224 => {:sha3_224, 28},
    :sha3_256 => {:sha3_256, 32},
    :sha3_384 => {:sha3_384, 48},
    :sha3_512 => {:sha3_512, 64}
  }
  @non_native_algo_map %{}

  @typedoc "Tracks the mapping from supported multihash codecs to their default digest lengths."
  @supported_algo_map @native_algo_map
                      |> Enum.map(fn {mh, {_, len}} -> {mh, len} end)
                      |> Enum.into(%{})
                      |> Map.merge(@non_native_algo_map)
  @typedoc "Tracks the supported multihash codecs."
  @supported_algos Map.keys(@supported_algo_map)

  @doc ~S"""
  Creates a `t:IPLD.Multiformatter.Multihasher.t/0` from a binary of Enumerable over binary data.

  ## Examples

      iex> IPLD.Multiformatter.Multihasher.from("Hello", :sha1)
      {:ok, %IPLD.Multiformatter.Multihasher{codec: :sha1, length: 20, digest: <<247, 255, 158, 139, 123, 178, 224, 155, 112, 147, 90, 93, 120, 94, 12, 197, 217, 208, 171, 240>>}}

      iex> IPLD.Multiformatter.Multihasher.from("Hello", :sha2_256)
      {:ok, %IPLD.Multiformatter.Multihasher{codec: :sha2_256, length: 32, digest: <<24, 95, 141, 179, 34, 113, 254, 37, 245, 97, 166, 252, 147, 139, 46, 38, 67, 6, 236, 48, 78, 218, 81, 128, 7, 209, 118, 72, 38, 56, 25, 105>>}}

      iex> IPLD.Multiformatter.Multihasher.from(["H", "e", "l", "l", "o"], :sha1, 10)
      {:ok, %IPLD.Multiformatter.Multihasher{codec: :sha1, length: 10, digest: <<247, 255, 158, 139, 123, 178, 224, 155, 112, 147>>}}

      iex> IPLD.Multiformatter.Multihasher.from(Stream.cycle(["H", "e", "l", "l", "o"]) |> Stream.take(5), :sha1, 10)
      {:ok, %IPLD.Multiformatter.Multihasher{codec: :sha1, length: 10, digest: <<247, 255, 158, 139, 123, 178, 224, 155, 112, 147>>}}

    Invalid `codec`, `trunc_length` corresponding to the hash function will return an error:

      iex> IPLD.Multiformatter.Multihasher.from("Hello", :sha2_unknow)
      {:error, :unsupported_hash_algo}

      iex> IPLD.Multiformatter.Multihasher.from("Hello", :sha1, 32)
      {:error, :invalid_truncating_length}
  """
  @spec from(
          blob :: Format.blob(),
          codec :: __MODULE__.multihash_codec(),
          trunc_length :: trunc_length
        ) :: on_from
  def from(blob, codec \\ @default_mh, trunc_length \\ @default_len)

  def from(_blob, codec, _trunc_length) when codec not in @supported_algos,
    do: {:error, :unsupported_hash_algo}

  def from(blob, codec, @default_len) do
    with {:ok, digest} <- digest(blob, codec, @default_len),
         do: from_digest(digest, codec, @default_len)
  end

  def from(blob, codec, trunc_length) do
    with {:ok, digest} <- digest(blob, codec, trunc_length),
         do: from_digest(digest, codec, trunc_length)
  end

  @doc ~S"""
  Creates a `t:IPLD.Multiformatter.Multihasher.t/0` from a multihash binary.

  ## Examples:

      iex> IPLD.Multiformatter.Multihasher.from_bytes(<<17, 20, 247, 255, 158, 139, 123, 178, 224, 155, 112, 147, 90, 93, 120, 94, 12, 197, 217, 208, 171, 240>>)
      {:ok, %IPLD.Multiformatter.Multihasher{codec: :sha1, length: 20, digest: <<247, 255, 158, 139, 123, 178, 224, 155, 112, 147, 90, 93, 120, 94, 12, 197, 217, 208, 171, 240>>}}

      iex> IPLD.Multiformatter.Multihasher.from_bytes(<<17, 10, 247, 255, 158, 139, 123, 178, 224, 155, 112, 147>>)
      {:ok, %IPLD.Multiformatter.Multihasher{codec: :sha1, length: 10, digest: <<247, 255, 158, 139, 123, 178, 224, 155, 112, 147>>}}

    Invalid multihashes decode into errors:

      iex> IPLD.Multiformatter.Multihasher.from_bytes(<<17, 20, 247, 255, 158, 139, 123, 178, 224, 155, 112, 147, 90, 93, 120, 94, 12, 197, 217, 208, 171>>)
      {:error, :invalid_size}

      iex> IPLD.Multiformatter.Multihasher.from_bytes(<<25, 20, 247, 255, 158, 139, 123, 178, 224, 155, 112, 147, 90, 93, 120, 94, 12, 197, 217, 208, 171, 240>>)
      {:error, :invalid_hash_code}

      iex> IPLD.Multiformatter.Multihasher.from_bytes(<<17, 32, 247, 255, 158, 139, 123, 178, 224, 155, 112, 147, 90, 93, 120, 94, 12, 197, 217, 208, 171, 240>>)
      {:error, :invalid_length}

      iex> IPLD.Multiformatter.Multihasher.from_bytes("Hello")
      {:error, :invalid_hash_code}
  """
  @spec from_bytes(multihash :: binary) :: on_from
  def from_bytes(multihash) when is_binary(multihash) do
    case Multihash.decode(multihash) do
      {:error, reason} ->
        {:error, convert_error(reason)}

      {:ok, %Multihash{name: mh, length: len, digest: digest}} ->
        {:ok, %__MODULE__{codec: mh, length: len, digest: digest}}
    end
  end

  @doc ~S"""
  Creates a `t:IPLD.Multiformatter.Multihasher.t/0` from a digest binary.

  ## Examples
      iex> IPLD.Multiformatter.Multihasher.from_digest(:crypto.hash(:sha, "Hello"), :sha1)
      {:ok, %IPLD.Multiformatter.Multihasher{codec: :sha1, length: 20, digest: <<247, 255, 158, 139, 123, 178, 224, 155, 112, 147, 90, 93, 120, 94, 12, 197, 217, 208, 171, 240>>}}

      iex> IPLD.Multiformatter.Multihasher.from_digest("1234567890123456789012345678901234567890123456789012345678901234", :sha3, 10)
      {:ok, %IPLD.Multiformatter.Multihasher{codec: :sha3, length: 10, digest: <<49, 50, 51, 52, 53, 54, 55, 56, 57, 48>>}}

      iex> IPLD.Multiformatter.Multihasher.from_digest(:crypto.hash(:sha256, "Hello"), :sha2_256)
      {:ok, %IPLD.Multiformatter.Multihasher{codec: :sha2_256, length: 32, digest: <<24, 95, 141, 179, 34, 113, 254, 37, 245, 97, 166, 252, 147, 139, 46, 38, 67, 6, 236, 48, 78, 218, 81, 128, 7, 209, 118, 72, 38, 56, 25, 105>>}}

    Invalid `hash_code`, `digest` length corresponding to the hash function will return an error

      iex> IPLD.Multiformatter.Multihasher.from_digest(:crypto.hash(:sha, "Hello"), :sha2_unknow)
      {:error, :invalid_multihash_codec}

      iex> IPLD.Multiformatter.Multihasher.from_digest(:crypto.hash(:sha, "Hello"), 0x20)
      {:error, :invalid_multihash_codec}

    It's possible to [truncate a digest](https://github.com/jbenet/multihash/issues/1#issuecomment-91783612) by passing an optional `length` parameter. Passing a `length` longer than the default digest length of the hash function will return an error.

      iex> IPLD.Multiformatter.Multihasher.from_digest(:crypto.hash(:sha, "Hello"), :sha1, 10)
      {:ok, %IPLD.Multiformatter.Multihasher{codec: :sha1, length: 10, digest: <<247, 255, 158, 139, 123, 178, 224, 155, 112, 147>>}}

      iex> IPLD.Multiformatter.Multihasher.from_digest(:crypto.hash(:sha, "Hello"), :sha1, 30)
      {:error, :invalid_truncating_length}
  """
  @spec from_digest(
          digest :: binary,
          codec :: __MODULE__.multihash_codec(),
          trunc_len :: __MODULE__.trunc_length()
        ) :: on_from
  def from_digest(digest, codec, trunc_len \\ @default_len)

  def from_digest(digest, codec, @default_len) do
    case Map.get(@supported_algo_map, codec) do
      nil -> {:error, :invalid_multihash_codec}
      len -> {:ok, %__MODULE__{codec: codec, length: len, digest: digest}}
    end
  end

  def from_digest(digest, codec, trunc_len) do
    default_trunc_len = Map.get(@supported_algo_map, codec, nil)

    if is_valid_trunc_len(default_trunc_len, trunc_len) do
      digest = Kernel.binary_part(digest, 0, trunc_len)
      {:ok, %__MODULE__{codec: codec, length: trunc_len, digest: digest}}
    else
      {:error, :invalid_truncating_length}
    end
  end

  @doc ~S"""
  Serializes a `t:IPLD.Multiformatter.Multihasher.t/0` struct into raw binary.
  """
  @spec to_bytes(multihasher :: __MODULE__.t()) :: on_to
  def to_bytes(%__MODULE__{codec: mh, length: len, digest: digest}) do
    case Multihash.encode(mh, digest, len) do
      {:error, reason} -> {:error, convert_error(reason)}
      {:ok, digest_bytes} -> {:ok, digest_bytes}
    end
  end

  # @doc ~S"""

  # """
  # @spec verify(blob :: Format.blob(), multihash :: binary) :: on_verify
  # def verify(blob, multihash) when is_binary(multihash) do
  #   {:error, :unimplemented}
  # end

  # -----

  @doc false
  @spec digest(
          blob :: Format.blob(),
          codec :: __MODULE__.multihash_codec(),
          length :: trunc_length
        ) :: binary
  defp digest(blob, codec, trunc_length \\ :default)

  Enum.each(@native_algo_map, fn {codec, {hash_code, hash_len}} ->
    defp digest(_blob, unquote(codec), trunc_length)
         when not is_valid_trunc_len(unquote(hash_len), trunc_length),
         do: {:error, :invalid_truncating_length}

    defp digest(blob, unquote(codec), _trunc_len) when is_binary(blob) or is_bitstring(blob),
      do: {:ok, do_native_digest(blob, unquote(hash_code))}

    defp digest(blob, unquote(codec), _trunc_len) when is_list(blob),
      do: {:ok, do_native_digest_enum(blob, unquote(hash_code))}

    defp digest(blob, unquote(codec), _trunc_len) do
      case Enumerable.impl_for(blob) do
        nil -> {:error, :invalid_blob}
        _ -> {:ok, do_native_digest_enum(blob, unquote(hash_code))}
      end
    end
  end)

  @doc false
  defp do_native_digest(binary, hash_code), do: :crypto.hash(hash_code, binary)

  @doc false
  defp do_native_digest_enum(enum, hash_code) do
    enum
    |> Enum.reduce(:crypto.hash_init(hash_code), &:crypto.hash_update(&2, &1))
    |> :crypto.hash_final()
  end

  @doc false
  defp convert_error(reason) do
    reason
    |> String.downcase()
    |> String.replace(" ", "_")
    |> String.to_atom()
  end
end
