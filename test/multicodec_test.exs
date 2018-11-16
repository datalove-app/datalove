defmodule MulticodecTest do
  use ExUnit.Case, async: true
  doctest Multicodec

  test "multicodec transparently encodes and decodes" do
    address = "http://3g2upl4pq6kufc4m.onion/"
    codec = "onion"
    assert Multicodec.encode!(address, codec) |> Multicodec.decode!() == address
    {:ok, encoded_data} = Multicodec.encode(address, codec)
    assert Multicodec.decode(encoded_data) == {:ok, address}
    assert Multicodec.encode!(address, codec) |> Multicodec.codec_decode!() == {address, codec}
    assert Multicodec.encode!(address, codec) |> Multicodec.codec_decode() == {:ok, {address, codec}}
  end

  test "encode!/2 raises errors" do
    assert_raise ArgumentError, fn -> Multicodec.encode!("chicken", <<>>) end
    assert_raise ArgumentError, fn -> Multicodec.encode!("chicken", "I'm digging for a fire") end
    assert_raise ArgumentError, fn -> Multicodec.encode!("chicken", "httpsbroccoli") end
    assert_raise ArgumentError, fn -> Multicodec.encode!("chicken", "22helensagreehttps") end
  end

  test "encode/2 handles errors" do
    assert {:error, _} = Multicodec.encode("chicken", <<>>)
    assert {:error, _} = Multicodec.encode("chicken", "I'm digging for a fire")
    assert {:error, _} = Multicodec.encode("chicken", "httpsbroccoli")
    assert {:error, _} = Multicodec.encode("chicken", "22helensagreehttps")
  end

  test "decode!/1 raises errors" do
    data = "עוגיות"
    assert_raise ArgumentError, fn -> Multicodec.decode!(data) end
  end

  test "decode/1 handles errors" do
    data = "עוגיות"
    assert {:error, _} = Multicodec.decode(data)
  end

  test "codec_decode!/1 raises errors" do
    data = "עוגיות"
    assert_raise ArgumentError, fn -> Multicodec.codec_decode!(data) end
  end

  test "codec_decode/1 handles errors" do
    data = "עוגיות"
    assert {:error, _} = Multicodec.codec_decode(data)
  end

  test "codec/1 raises errors" do
    data = "עוגיות"
    assert_raise ArgumentError, fn -> Multicodec.codec!(data) end
  end

  test "codec/1 handles errors" do
    data = "עוגיות"
    assert {:error, _} = Multicodec.codec(data)
  end

  test "encodes empty strings for all codecs" do
    for codec <- Multicodec.codecs() do
      {:ok, paranoid_encoded} = Multicodec.encode(<<>>, codec)
      encoded_data = Multicodec.encode!(<<>>, codec)
      assert Multicodec.codec!(encoded_data) == codec
      assert Multicodec.codec!(paranoid_encoded) == codec
    end
  end

  test "encodes for all codecs" do
    data = "Like strawberries and cream, it's the only way, it's the only way to be."
    for codec <- Multicodec.codecs() do
      assert {:ok, _} = Multicodec.encode(data, codec)
      assert Multicodec.encode!(data, codec) |> is_binary() == true
    end
  end

  test "encodes transparently for all codecs" do
    data = "Massive, huge, really big"
    for codec <- Multicodec.codecs() do
      assert Multicodec.encode!(data, codec) |> Multicodec.decode!() == data
      {:ok, encoded_data} = Multicodec.encode(data, codec)
      assert Multicodec.decode(encoded_data) == {:ok, data}
      assert Multicodec.encode!(data, codec) |> Multicodec.codec_decode!() == {data, codec}
      assert Multicodec.encode!(data, codec) |> Multicodec.codec_decode() == {:ok, {data, codec}}
    end
  end

  test "prefix_for returns the correct prefix" do
    codec = "https"
    prefix = <<187, 3>>
    assert Multicodec.prefix_for!(codec) == prefix
    assert Multicodec.prefix_for(codec) == {:ok, prefix}
  end

  test "prefix_for!/1 raises errors" do
    assert_raise ArgumentError, fn -> Multicodec.prefix_for!("") end
    assert_raise ArgumentError, fn -> Multicodec.prefix_for!("httpsteakfries") end
    assert_raise ArgumentError, fn -> Multicodec.prefix_for!("cheesesaucehttps") end
  end

  test "prefix_for!/1 handles errors" do
    assert {:error, _reason} = Multicodec.prefix_for("")
    assert {:error, _reason} = Multicodec.prefix_for("httpsteakfries")
    assert {:error, reason} = Multicodec.prefix_for("cheesesaucehttps")
  end

  test "codecs/0 returns a list of codecs" do
    codecs = Multicodec.codecs()
    assert is_list(codecs) == true
    assert Enum.count(codecs) > 0
    assert Enum.all?(codecs, &String.valid?/1) == true
  end

  test "codec returns the correct codec for all codecs" do
    data = "I used to sometimes try to catch her, but never even caught her name"
    for codec <- Multicodec.codecs() do
      {:ok, encoded_data} = Multicodec.encode(data, codec)
      assert Multicodec.codec!(encoded_data) == codec
      assert Multicodec.codec(encoded_data) == {:ok, codec}
    end
  end

  test "mappings/0 returns all mappings" do
    mappings = Multicodec.mappings()
    assert is_list(mappings) == true
    assert Enum.count(mappings) > 0
    assert Enum.all?(mappings, &is_map/1) == true
  end

end
