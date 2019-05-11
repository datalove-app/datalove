defmodule IPLD do
  @moduledoc File.read!(__DIR__ <> "/../README.md")

  def resolve(cid, path), do: nil
  def tree(cid, path \\ "", opts \\ []), do: nil

  def get(cid), do: nil
  def get_many(cids), do: nil

  def put(node, format, opts \\ []), do: nil
  def put_many(node, format, opts \\ []), do: nil

  def remove(cid), do: nil
  def remove_many(cids), do: nil
end
