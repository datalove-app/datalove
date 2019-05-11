defmodule IPLD.Node do
  @moduledoc """
  Typedefs for an IPLD Node.
  """

  # TODO: is this right?
  @type link :: CID.t() | __MODULE__.t()
  @type ld_list :: [__MODULE__.t()]
  @type ld_map :: %{
          optional(:__struct__) => atom,
          optional(scalar) => __MODULE__.t()
        }

  @type scalar :: nil | boolean | integer | float | String.t() | binary | link
  @type recursive :: ld_list | ld_map
  @type t :: scalar | recursive
end
