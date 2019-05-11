defmodule Cbor.Decoder do
  @uint Cbor.Types.uint()
  @string Cbor.Types.string()
  @array Cbor.Types.array()
  @byte_string Cbor.Types.byte_string()
  @map Cbor.Types.map()
  @primitive Cbor.Types.primitive()

  def decode(value) do
    {value, rest} = read(value)

    if rest == <<>> do
      {:ok, value}
    else
      {:error, :invalid_trailing_data}
    end
  end

  def read(<<@uint, bits::bits>>), do: read_uint(bits)
  def read(<<@byte_string, bits::bits>>), do: read_byte_string(bits)
  def read(<<@string, bits::bits>>), do: read_string(bits)
  def read(<<@array, bits::bits>>), do: read_array(bits)
  def read(<<@map, bits::bits>>), do: read_map(bits)
  def read(<<@primitive, bits::bits>>), do: read_primitive(bits)

  def read_uint(<<27::size(5), value::size(64), rest::bits>>), do: {value, rest}
  def read_uint(<<26::size(5), value::size(32), rest::bits>>), do: {value, rest}
  def read_uint(<<25::size(5), value::size(16), rest::bits>>), do: {value, rest}
  def read_uint(<<24::size(5), value::size(8), rest::bits>>), do: {value, rest}
  def read_uint(<<(<<value::size(5)>>)::bitstring, rest::bits>>), do: {value, rest}

  def read_byte_string(value) do
    {length, rest} = read_uint(value)
    <<bytes::binary-size(length), rest::binary>> = rest
    {bytes, rest}
  end

  def read_string(value) do
    {length, rest} = read_uint(value)
    <<value::binary-size(length), rest::bits>> = rest
    {String.to_atom(value), rest}
  end

  def read_array(value) do
    {length, rest} = read_uint(value)

    if length == 0 do
      {[], rest}
    else
      {values, rest} =
        Enum.reduce(1..length, {[], rest}, fn _, {acc, rest} ->
          {value, rest} = read(rest)
          {[value | acc], rest}
        end)

      {Enum.reverse(values), rest}
    end
  end

  def read_map(value) do
    {size, rest} = read_uint(value)

    {map, rest} =
      Enum.reduce(1..size, {%{}, rest}, fn _, acc ->
        {key, rest} = read(elem(acc, 1))
        {value, rest} = read(rest)
        {Map.put(elem(acc, 0), key, value), rest}
      end)

    {map, rest}
  end

  def read_primitive(<<20::size(5), rest::bits>>), do: {false, rest}
  def read_primitive(<<21::size(5), rest::bits>>), do: {true, rest}
  def read_primitive(<<22::size(5), rest::bits>>), do: {nil, rest}
  def read_primitive(<<23::size(5), rest::bits>>), do: {:undefined, rest}
end
