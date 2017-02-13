defmodule Wwwtech.Mixfile do
  use Mix.Project

  def project do
    [app: :wwwtech,
     version: "0.0.17",
     elixir: "~> 1.2",
     elixirc_paths: elixirc_paths(Mix.env),
     compilers: [:phoenix, :gettext] ++ Mix.compilers,
     build_embedded: Mix.env == :prod,
     start_permanent: Mix.env == :prod,
     aliases: aliases(),
     deps: deps()]
  end

  # Configuration for the OTP application.
  #
  # Type `mix help compile.app` for more information.
  def application do
    [mod: {Wwwtech, []},
     applications: [:phoenix, :phoenix_html, :cowboy, :logger, :gettext,
                    :phoenix_ecto, :postgrex, :tzdata, :httpotion, :bamboo,
                    :bamboo_smtp, :cmark, :comeonin, :elixir_exif, :floki,
                    :logger_file_backend, :microformats2, :mogrify,
                    :phoenix_pubsub, :scrivener, :scrivener_ecto, :scrivener_html,
                    :timex, :timex_ecto, :webmentions, :edeliver]]
  end

  # Specifies which paths to compile per environment.
  defp elixirc_paths(:test), do: ["lib", "web", "test/support"]
  defp elixirc_paths(_),     do: ["lib", "web"]

  # Specifies your project dependencies.
  #
  # Type `mix help deps` for examples and options.
  defp deps do
    [{:phoenix, "~> 1.2.1"},
     {:phoenix_pubsub, "~> 1.0"},
     {:phoenix_ecto, "~> 3.0"},
     {:postgrex, ">= 0.0.0"},
     {:phoenix_html, "~> 2.6"},
     {:phoenix_live_reload, "~> 1.0", only: :dev},
     {:gettext, "~> 0.11"},
     {:comeonin, "~> 3.0"},
     {:cmark, "~> 0.5"},
     {:scrivener, ">= 0.0.0"},
     {:scrivener_ecto, "~> 1.0"},
     {:scrivener_html, ">= 0.0.0"},
     {:mogrify, github: "route/mogrify"},
     {:timex, "~> 3.0"},
     {:timex_ecto, "~> 3.0"},
     {:httpotion, "~> 3.0"},
     {:floki, "~> 0.7.2"},
     {:cowboy, "~> 1.0"},
     {:elixir_exif, github: "ckruse/ElixirExif"},
     {:logger_file_backend, github: "ckruse/logger_file_backend"},
     {:microformats2, "~> 0.0.3"},
     {:webmentions, "~> 0.2.0"},
     {:bamboo, "~> 0.8"},
     {:bamboo_smtp, "~> 1.3.0"},
     {:distillery, "~> 1.0"},
     {:edeliver, "~> 1.4.0"}]
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
