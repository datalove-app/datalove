defmodule Cbor.Types do
  @uint <<0b000::3>>
  @byte_string <<0b010::3>>
  @string <<0b011::3>>
  @array <<0b100::3>>
  @map <<0b101::3>>
  @primitive <<0b111::3>>

  def uint, do: @uint
  def byte_string, do: @byte_string
  def string, do: @string
  def array, do: @array
  def map, do: @map
  def primitive, do: @primitive
end
