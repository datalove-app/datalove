defmodule Cbor do
  alias Cbor.Encoder
  alias Cbor.Decoder

  def encode(value) do
    Encoder.encode(value)
  end

  def decode!(value) do
    {:ok, result} = decode(value)

    result
  end

  def decode(value) do
    Decoder.decode(value)
  end
end
