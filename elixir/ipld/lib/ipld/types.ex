defmodule IPLD.Types do
  @moduledoc """
  Typedefs for IPLD objects.
  """

  alias IPLD.Types.Dag

  @typedoc "Abstract type that contains a single binary or can produce chunks of a single binary."
  @type blob :: iodata | Enumerable.t(iodata)
  @typedoc "Abstract type that contains a native representation of an IPLD dag, or can or produce chunks of one."
  @type dag :: Dag.t | Enumerable.t(Dag.t)
end
