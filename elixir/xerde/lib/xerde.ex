defmodule Xerde do
  @moduledoc File.read!(__DIR__ <> "/../README.md")

  use Rustler, crate: :xerde

  def serialize(term_stream), do: erlang.nif_error(:nif_not_loaded)

  def serialize_stream(term_stream), do: Stream.map(term_stream, &serialize/1)

  def serialize_seq_stream(term_seq_stream) do
    # let rows = /* whatever iterator */;
    # TODO: how do you want the memory to be handled/cleared?
    # TODO: implement a shared memory writer for rustler (Binary/OwnedBinary writer and reader)
    # TODO: or just write to a BufWriter and read within the serialize_seq_element NIF
    # let out = std::io::stdout();
    #
    # let mut ser = serde_json::Serializer::new(out);
    # let mut seq = ser.serialize_seq(Some(rows.len()))?; // or None if unknown
    # for row in rows {
    #     seq.serialize_element(&row)?;
    # }
    # seq.end()?;

    Stream.transform(
      term_seq_stream,
      fn -> _start_serialize_seq(:json, 10) end,
      fn data, seq ->
        _serialize_seq_element(seq, data)
        # TODO: fetch serialized bytes, return them from this fn
        bytes = nil
        {bytes, seq}
      end,
      fn seq -> _end_serialize_seq(seq) end
    )
  end

  def deserialize(bytes), do: erlang.nif_error(:nif_not_loaded)

  def deserialize_stream(byte_stream), do: Stream.map(byte_stream, &deserialize/1)

  def deserialize_seq_stream(byte_seq_stream) do
  end

  defp _start_serialize_seq(), do: erlang.nig_error(:nif_not_loaded)
  defp _serialize_seq_element(), do: erlang.nig_error(:nif_not_loaded)
  defp _end_serialize_seq(), do: erlang.nig_error(:nif_not_loaded)

  defp _start_deserialize_seq(), do: erlang.nig_error(:nif_not_loaded)
  defp _deserialize_seq_element(), do: erlang.nig_error(:nif_not_loaded)
  defp _end_deserialize_seq(), do: erlang.nig_error(:nif_not_loaded)
end
