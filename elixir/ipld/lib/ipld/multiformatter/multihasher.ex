defmodule IPLD.Multiformatter.Multihasher do
  @moduledoc """
  Module for abstracting hashing over some of the various [Multihash](https://github.com/multiformats/multihash) algorithms.

  Uses [:ex_multihash](https://github.com/multiformats/ex_multihash) and borrows from [:multihashing](https://github.com/candeira/ex_multihashing).
  """

  alias Multihash
  alias IPLD.Format
  import IPLD.Multiformatter.Util

  @type t :: %__MODULE__{
          codec: __MODULE__.multihash_codec(),
          length: __MODULE__.trunc_length(),
          digest: binary
        }
  @type multihash_codec :: Multihash.hash_type()
  @type trunc_length :: :default | non_neg_integer

  @type on_from :: {:ok, Multihasher.t()} | {:error, reason :: atom}
  @type on_to :: {:ok, Format.blob()} | {:error, reason :: atom}
  @type on_verify :: {:ok, bool} | {:error, reason :: atom}

  @enforce_keys [:codec, :length, :digest]
  defstruct codec: @default_mh,
            length: @default_len,
            digest: <<>>

  @default_mh :sha2_256
  @default_len :default

  @doc "Tracks a mapping from multihash codecs to hash codes used by `:crypto.hash/2`, as well as their default digest lengths."
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
  @supported_algo_map Map.merge(@native_algo_map, @non_native_algo_map)
  @supported_algos Map.keys(@native_algo_map) ++ Map.keys(@non_native_algo_map)

  @doc ~S"""
  Creates a `t:IPLD.Multiformatter.Multihasher.t/0` from a binary of Enumerable over binary data.

  ## Examples

      iex> IPLD.Multiformatter.Multihasher.from("Hello", :sha1)
      {:ok, %IPLD.Multiformatter.Multihasher{codec: :sha1, length: 20, digest: <<247, 255, 158, 139, 123, 178, 224, 155, 112, 147, 90, 93, 120, 94, 12, 197, 217, 208, 171, 240>>}}

      iex> IPLD.Multiformatter.Multihasher.from("Hello", :sha2_256)
      {:ok, %IPLD.Multiformatter.Multihasher{codec: :sha2_256, length: 32, digest: <<24, 95, 141, 179, 34, 113, 254, 37, 245, 97, 166, 252, 147, 139, 46, 38, 67, 6, 236, 48, 78, 218, 81, 128, 7, 209, 118, 72, 38, 56, 25, 105>>}}

      iex> IPLD.Multiformatter.Multihasher.from(["H", "e", "l", "l", "o"], :sha1, 10)
      {:ok, %IPLD.Multiformatter.Multihasher{codec: :sha1, length: 10, digest: <<247, 255, 158, 139, 123, 178, 224, 155, 112, 147>>}}
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
      {_, len} -> {:ok, %__MODULE__{codec: codec, length: len, digest: digest}}
    end
  end

  def from_digest(digest, codec, trunc_len) do
    default_trunc_len = Map.get(@supported_algo_map, codec) |> elem(1)

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
      case Enumerable.impl_for(unquote(codec)) do
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
