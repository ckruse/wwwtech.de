defmodule Wwwtech.Mixfile do
  use Mix.Project

  def project do
    [app: :wwwtech,
     version: "0.0.1",
     elixir: "~> 1.0",
     elixirc_paths: elixirc_paths(Mix.env),
     compilers: [:phoenix, :gettext] ++ Mix.compilers,
     build_embedded: Mix.env == :prod,
     start_permanent: Mix.env == :prod,
     aliases: aliases,
     deps: deps]
  end

  # Configuration for the OTP application.
  #
  # Type `mix help compile.app` for more information.
  def application do
    [mod: {Wwwtech, []},
     applications: [:phoenix, :phoenix_html, :cowboy, :logger, :gettext,
                    :phoenix_ecto, :postgrex, :tzdata, :httpotion]]
  end

  # Specifies which paths to compile per environment.
  defp elixirc_paths(:test), do: ["lib", "web", "test/support"]
  defp elixirc_paths(_),     do: ["lib", "web"]

  # Specifies your project dependencies.
  #
  # Type `mix help deps` for examples and options.
  defp deps do
    [{:phoenix, "~> 1.1"},
     {:phoenix_ecto, "~> 2.0"},
     {:postgrex, ">= 0.0.0"},
     {:phoenix_html, "~> 2.3"},
     {:phoenix_live_reload, "~> 1.0", only: :dev},
     {:gettext, "~> 0.9"},
     {:comeonin, "~> 1.0"},
     {:cmark, "~> 0.5"},
     {:scrivener, ">= 0.0.0"},
     {:scrivener_html, ">= 0.0.0"},
     {:mogrify, github: "ckruse/mogrify"},
     {:timex, "~> 0.19"},
     {:httpotion, "~> 2.2"},
     {:floki, "~> 0.7.2"},
     {:gen_smtp, "~> 0.9.0"},
     {:cowboy, "~> 1.0"},
     {:ecto, "~> 1.1.2"},
     {:logger_file_backend, github: "onkel-dirtus/logger_file_backend"},
     {:elixir_exif, github: "ckruse/ElixirExif"},
     {:microformats2, "~> 0.0.3"},
     {:webmentions, "~> 0.0.5"}]
  end

  # Aliases are shortcut or tasks specific to the current project.
  # For example, to create, migrate and run the seeds file at once:
  #
  #     $ mix ecto.setup
  #
  # See the documentation for `Mix` for more info on aliases.
  defp aliases do
    ["ecto.setup": ["ecto.create", "ecto.migrate", "run priv/repo/seeds.exs"],
     "ecto.reset": ["ecto.drop", "ecto.setup"]]
  end
end
