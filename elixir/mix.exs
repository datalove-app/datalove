defmodule Monorepo.Mixfile do
  @moduledoc false

  use Mix.Project

  @name         :monorepo
  @version      "0.0.1-dev"
  @description  """
  """
  @github       "https://github.com/datalove-app/datalove"
  @files        ["mix.exs", "mix.lock", "lib", "test", "README.md"]
  @maintainers  ["Sunny G"]
  @licenses     ["MIT"]

  # ------------------------------------------------------------

  def project do
    in_production = Mix.env == :prod

    [ app:              @name,
      version:          @version,
      description:      @description,
      elixir:           "~> 1.8",
      docs:             docs(),
      package:          package(),
      deps:             deps() ++ dev_deps(),
      build_embedded:   in_production,
      start_permanent:  in_production,
    ]
  end

  defp deps() do
    [ {:ipld,           path: __DIR__ <> "/ipld"},
      {:rustler,        "~> 0.20.0"},
    ]
  end

  defp dev_deps() do
    [ {:credo,          "~> 1.0.0", only: [:dev, :test], runtime: false},
      {:dialyxir,       "~> 0.5",   only: [:dev],        runtime: false},
      {:ex_doc,         "~> 0.19",  only: [:dev],        runtime: false},
      {:inch_ex, github: "rrrene/inch_ex", only: [:dev, :test], runtime: false},
      {:mix_test_watch, "~> 0.8",   only: [:dev],        runtime: false},
    ]
  end

  defp package do
    [ name:        @name,
      files:       @files,
      maintainers: @maintainers,
      licenses:    @licenses,
      links:       %{
        "GitHub" => @github,
      },
    ]
  end

  defp docs do
    [ main:       "readme",
      source_url: @github,
      extras:     ["README.md"],
    ]
  end
end