defmodule Cbor.MixProject do
  use Mix.Project

  def project do
    [
      app: :cbor,
      version: "0.1.7",
      description: "RFC 7049 Concise Binary Object Representation (CBOR) ",
      package: [
        maintainers: ["Mason Fischer"],
        licenses: ["MIT"],
        links: %{"GitHub" => "https://github.com/masonforest/elixir-cbor"},
      ],
      elixir: "~> 1.7",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:ex_doc, ">= 0.0.0", only: :dev}
    ]
  end
end
