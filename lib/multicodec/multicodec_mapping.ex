defmodule Multicodec.MulticodecMapping do
  @moduledoc false

  @enforce_keys [:codec, :code, :prefix]

  defstruct [:code, :codec, :prefix]

  @typedoc """
  A codec used to encode a binary as a Multicodec.
  """
  @type multi_codec() :: atom()

  @typedoc """
  A binary representation of a multicodec code encoded as an unsigned varint.
  """
  @type prefix() :: binary()

  @type t :: %__MODULE__{
          codec: multi_codec(),
          code: non_neg_integer(),
          prefix: prefix()
        }

  @spec new(multi_codec(), non_neg_integer()) :: t()
  def new(codec, code) when is_atom(codec) and is_integer(code) do
    prefix = prefix(code)
    struct(%__MODULE__{codec: codec, code: code, prefix: prefix})
  end

  @doc """
  Encodes a Multicodec code as an unsigned varint prefix.
  """
  @spec prefix(non_neg_integer()) :: prefix()
  def prefix(code) do
    Varint.LEB128.encode(code)
  end

  @doc """
  Decodes a Multicodec prefix into a code.
  """
  @spec code(prefix()) :: non_neg_integer()
  def code(prefix) do
    {code, _data} = Varint.LEB128.decode(prefix)
    code
  end
end
