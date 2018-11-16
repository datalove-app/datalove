defmodule Multicodec.MixProject do
  use Mix.Project

  @version "0.0.1"
  @source_url "https://github.com/nocursor/ex-multicodec"

  def project do
    [
      app: :multicodec,
      version: @version,
      elixir: ">= 1.7.0",
      start_permanent: Mix.env() == :prod,
      deps: deps(),

      # Hex
      description: "Elixir library for encoding and decoding data using the Multicodec standard.",
      package: package(),

      # Docs
      source_url: @source_url,
      docs: docs()
    ]
  end

  def application do
    [
    ]
  end

  defp package do
    [
      maintainers: [
        "nocursor",
      ],
      licenses: ["MIT"],
      links: %{github: @source_url},
      files: ~w(lib NEWS.md LICENSE.md mix.exs README.md)
    ]
  end

  defp docs do
    [
      source_ref: "v#{@version}",
      main: "readme",
      extra_section: "PAGES",
      extras: extras(),
      groups_for_extras: groups_for_extras()
    ]
  end

  defp extras do
    [
      "README.md",
      "docs/FAQ.md",
    ]
  end

  defp groups_for_extras do
    [
    ]
  end

  defp deps do
    [
      {:benchee, "~> 0.13.2", only: [:dev]},
      {:dialyxir, "~> 1.0.0-rc.3", only: [:dev], runtime: false},
      {:ex_doc, "~> 0.13", only: [:dev], runtime: false},
      {:varint, "~> 1.1"},
      {:csv, "~> 2.1"}
    ]
  end
end
