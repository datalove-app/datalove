defmodule Cbor.Encoder do
  alias Cbor.Types
  alias Cbor.Sorting

  def encode(value) when is_nil(value) or is_boolean(value) or value == :undefined,
    do: concat(Types.primitive(), encode_primitive(value))
  def encode(value) when is_integer(value),
    do: concat(Types.uint(), encode_uint(value))
  def encode(value) when is_atom(value),
    do: concat(Types.string(), encode_string(value))
  def encode(value) when is_binary(value),
    do: concat(Types.byte_string(), encode_byte_string(value))
  def encode(value) when is_list(value),
    do: concat(Types.array(), encode_array(value))
  def encode(value) when is_map(value),
    do: concat(Types.map(), encode_map(value))

  def concat(left, right) do
    <<left::bitstring, right::bitstring>>
  end

  def encode_byte_string(value) do
    length = encode_uint(byte_size(value))

    concat(length, value)
  end

  def encode_array(value) do
    length = encode_uint(length(value))
    values = Enum.map(value, &encode/1) |> Enum.join()

    concat(length, values)
  end

  def encode_map(value) do
    length = encode_uint(map_size(value))

    values =
      value
      |> Map.keys()
      |> Enum.sort(&Sorting.compare/2)
      |> Enum.map(fn key -> concat(encode(key), encode(value[key])) end)
      |> Enum.reduce(<<>>, &concat/2)

    concat(length, values)
  end

  def encode_string(value) do
    string = to_string(value)
    length = encode_uint(String.length(string))
    concat(length, string)
  end

  def encode_uint(value) when value in 0..23, do: <<value::5>>
  def encode_uint(value) when value in 24..0x100, do: <<24::size(5), value>>
  def encode_uint(value) when value in 0x101..0x10000,
    do: <<25::size(5), value::size(16)>>
  def encode_uint(value) when value in 0x10001..0x100000000,
    do: <<26::size(5), value::size(32)>>
  def encode_uint(value) when value in 0x100000001..0x10000000000000000,
    do: <<27::size(5), value::size(64)>>

  def encode_primitive(false), do: <<20::size(5)>>
  def encode_primitive(true), do: <<21::size(5)>>
  def encode_primitive(nil), do: <<22::size(5)>>
  def encode_primitive(:undefined), do: <<23::size(5)>>
end
