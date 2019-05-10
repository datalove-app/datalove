defmodule Cbor.Encoder do
  alias Cbor.Types
  alias Cbor.Sorting

  def encode(value) do
      case value do
        value when is_nil(value) or is_boolean(value) or value == :undefined ->
          concat(Types.primitive, encode_primitive(value))
        value when is_integer(value) ->
          concat(Types.unsigned_integer, encode_unsigned_int(value))
        value when is_atom(value) ->
          concat(Types.string, encode_string(value))
        value when is_binary(value) ->
          concat(Types.byte_string, encode_byte_string(value))
        value when is_list(value) ->
          concat(Types.array, encode_array(value))
        value when is_map(value) ->
          concat(Types.map, encode_map(value))
      end
  end

  def concat(left, right) do
    <<left::bitstring, right::bitstring>>
  end

  def encode_byte_string(value) do
    length = encode_unsigned_int(byte_size(value))

    concat(length, value)
  end

  def encode_array(value) do
    length = encode_unsigned_int(length(value))
    values =  Enum.map(value, &encode/1) |> Enum.join

    concat(length, values)
  end

  def encode_map(value) do
    length = encode_unsigned_int(map_size(value))
    values =  value
      |> Map.keys()
      |> Enum.sort(&Sorting.compare/2)
      |> Enum.map(fn(key) ->
        concat(encode(key), encode(value[key]))
      end)
      |> Enum.reduce(<<>>, &concat/2)

    concat(length, values)
  end

  def encode_string(value) do
    string = to_string(value)
    length = encode_unsigned_int(String.length(string))
    concat(length, string)
  end

  def encode_unsigned_int(value) do
    case value do
      value when value in 0..23 ->
        <<value::5>>
      value when value in 24..0x100 ->
        <<24::size(5), value>>
      value when value in 0x101..0x10000 ->
        <<25::size(5), value::size(16)>>
      value when value in 0x10001..0x100000000 ->
        <<26::size(5), value::size(32)>>
      value when value in 0x100000001..0x10000000000000000 ->
        <<27::size(5), value::size(64)>>
    end
  end

  def encode_primitive(value) do
    case value do
      false -> <<20::size(5)>>
      true -> <<21::size(5)>>
      nil -> <<22::size(5)>>
      :undefined -> <<23::size(5)>>
    end
  end
end
