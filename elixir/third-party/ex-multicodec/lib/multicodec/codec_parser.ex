defmodule Multicodec.CodecParser do
  @moduledoc false

  alias Multicodec.MulticodecMapping

  @default_table_path Path.join(:code.priv_dir(:multicodec), "table.csv")

  @doc """
  Parses a codec table from the Multicodec's official CSV file.
  """
  @spec parse_table(Path.t()) :: [MulticodecMapping.t()]
  def parse_table(path \\ @default_table_path) do
    File.stream!(path)
    |> CSV.decode(headers: true, strip_fields: true)
    |> Enum.flat_map(fn
      # there are placeholders
      {:ok, %{"code" => "0x"}} ->
        []

      # this is the same as <<0>>
      {:ok, %{"code" => "NUL"}} ->
        []

      # these happen when there's bad data
      {:ok, %{"code" => <<>>}} ->
        []

      {:ok, %{"codec" => <<>>}} ->
        []

      # finally acceptable data
      {:ok, %{"code" => code, "codec" => codec}} ->
        parsed_code = parse_code(code)

        codec =
          codec
          |> String.replace("-", "_")
          |> String.to_atom()

        [MulticodecMapping.new(codec, parsed_code)]

      _ ->
        []
    end)
  end

  @doc """
  Puts the codec table from the Multicodec's official CSV file to the given device.
  """
  @spec inspect_table(atom() | pid()) :: :ok
  def inspect_table(device \\ :stdio, path \\ @default_table_path) do
    codec_data =
      parse_table(path)
      |> (fn mapping ->
            [
              ?[,
              Enum.map(mapping, &inspect(&1, limit: :infinity))
              |> Enum.intersperse(",\n"),
              ?]
            ]
          end).()

    IO.puts(device, codec_data)
  end

  defp parse_code(<<"0x", rest::binary>>) do
    {n, _} = Integer.parse(rest, 16)
    n
  end

  defp parse_code(data) when is_binary(data) do
    :binary.decode_unsigned(data)
  end

  defp parse_code(code) when is_integer(code) do
    code
  end

  defp parse_code(code) do
    raise ArgumentError, "Found non-binary code: #{inspect(code)}"
  end
end
