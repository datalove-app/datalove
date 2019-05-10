defmodule Cbor.Decoder do
  @unsigned_integer Cbor.Types.unsigned_integer()
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

  def read(value) do
    case value do
      << @unsigned_integer, bits::bits >> ->
        read_unsigned_integer(bits)
      << @string, bits::bits >> ->
        read_string(bits)
      << @byte_string, bits::bits >> ->
        read_byte_string(bits)
      << @array, bits::bits >> ->
        read_array(bits)
      << @map, bits::bits >> ->
        read_map(bits)
      << @primitive, bits::bits >> ->
        read_primitive(bits)
    end

  end

  def read_map(value) do
    {size, rest} = read_unsigned_integer(value)
    {map, rest} = Enum.reduce(1..size, {%{}, rest}, fn(_, acc) ->
      {key, rest} = read(elem(acc, 1))
      {value, rest} = read(rest)
      {Map.put(elem(acc, 0), key, value), rest}
    end)

    {map, rest}
  end

  def read_byte_string(value) do
    {length, rest} = read_unsigned_integer(value)
    <<bytes::binary-size(length), rest::binary>> = rest
    {bytes, rest}
  end

  def read_array(value) do
    {length, rest} = read_unsigned_integer(value)

    if length == 0 do
      {[], rest}
    else
     {values, rest} = Enum.reduce(1..length, {[], rest}, fn(_, {acc, rest}) ->
        {value, rest} = read(rest)
        {[value | acc], rest}
      end)
      {values |> Enum.reverse, rest}
    end
  end



  def read_string(value) do
    {length, rest} = read_unsigned_integer(value)
    << value::binary-size(length), rest::bits >> = rest
    {String.to_atom(value), rest}
  end

  def read_unsigned_integer(value) do
    case value do
      << 27::size(5), value::size(64), rest::bits >> ->
        {value, rest}
      << 26::size(5), value::size(32), rest::bits >> ->
        {value, rest}
      << 25::size(5), value::size(16), rest::bits >> ->
        {value, rest}
      << 24::size(5), value::size(8), rest::bits >> ->
        {value, rest}
      << <<value::size(5)>>::bitstring, rest::bits >> ->
        {value, rest}
    end
  end

  def read_primitive(value) do
    case value do
      << 20::size(5), rest::bits >> ->
        {false, rest}
      << 21::size(5), rest::bits >> ->
        {true, rest}
      << 22::size(5), rest::bits >> ->
        {nil, rest}
      << 23::size(5), rest::bits >> ->
        {:undefined, rest}
    end
  end
end
