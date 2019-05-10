defmodule CborTest do
  use ExUnit.Case
  doctest Cbor

  test "unsigned integers" do
    round_trip(1)
    round_trip(24)
    round_trip(42)
  end

  test "strings (symbols)" do
    round_trip(:test)
  end

  test "arrays" do
    round_trip([])
    round_trip([3,2,1])
    round_trip([100])
  end

  test "bytes" do
    round_trip(<<1,2,3>>)
  end

  test "maps" do
    round_trip(%{key1: :value1, key2: :value2})
  end

  test "primatives" do
    round_trip(nil)
    round_trip(true)
    round_trip(false)
    round_trip(:undefined)
  end

  test "invalid data" do
    assert {:error, :invalid_trailing_data} == Cbor.decode(<<1,2,3>>)
  end

  def round_trip(value) do
    assert value == Cbor.decode!(Cbor.encode(value))
  end
end
