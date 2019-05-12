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

defmodule IPLD.Types.Dag do
  @moduledoc """
  Typedefs for IPLD nodes.
  """

  # TODO: is this right?
  @typedoc false
  @type t :: scalar | recursive
  @typedoc false
  @type link :: CID.t() | __MODULE__.t()
  @type ld_list :: [__MODULE__.t()]
  @type ld_map :: %{
          optional(:__struct__) => atom,
          optional(scalar) => __MODULE__.t()
        }
  @type scalar :: nil | boolean | integer | float | String.t() | binary | link
  @type recursive :: ld_list | ld_map
end
