defmodule Wwwtech.Mixfile do
  use Mix.Project

  def project do
    [
      app: :wwwtech,
      version: "0.2.5",
      elixir: "~> 1.5",
      elixirc_paths: elixirc_paths(Mix.env()),
      compilers: [:phoenix, :gettext] ++ Mix.compilers(),
      build_embedded: Mix.env() == :prod,
      start_permanent: Mix.env() == :prod,
      aliases: aliases(),
      deps: deps()
    ]
  end

  # Configuration for the OTP application.
  #
  # Type `mix help compile.app` for more information.
  def application do
    [
      mod: {Wwwtech, []},
      applications: [
        :phoenix,
        :phoenix_html,
        :cowboy,
        :logger,
        :gettext,
        :phoenix_ecto,
        :postgrex,
        :tzdata,
        :httpotion,
        :bamboo,
        :bamboo_smtp,
        :cmark,
        :comeonin,
        :bcrypt_elixir,
        :elixir_exif,
        :floki,
        :microformats2,
        :mogrify,
        :phoenix_pubsub,
        :timex,
        :timex_ecto,
        :webmentions,
        :edeliver
      ]
    ]
  end

  # Specifies which paths to compile per environment.
  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_), do: ["lib"]

  # Specifies your project dependencies.
  #
  # Type `mix help deps` for examples and options.
  defp deps do
    [
      {:phoenix, "~> 1.3.0"},
      {:phoenix_pubsub, "~> 1.0"},
      {:phoenix_ecto, "~> 3.2"},
      {:postgrex, ">= 0.0.0"},
      {:phoenix_html, "~> 2.6"},
      {:phoenix_live_reload, "~> 1.0", only: :dev},
      {:gettext, "~> 0.11"},
      {:comeonin, "~> 4.0"},
      {:bcrypt_elixir, "~> 1.0"},
      {:cmark, "~> 0.5"},
      {:mogrify, github: "route/mogrify"},
      {:timex, "~> 3.1"},
      {:timex_ecto, github: "bitwalker/timex_ecto", tag: "3.2.0"},
      {:httpotion, "~> 3.0"},
      {:floki, "~> 0.15"},
      {:cowboy, "~> 1.0"},
      {:elixir_exif, github: "sschneider1207/ElixirExif"},
      {:microformats2, "~> 0.1.0"},
      {:webmentions, "~> 0.3"},
      {:bamboo, "~> 0.8"},
      {:bamboo_smtp, "~> 1.3"},
      {:distillery, "~> 1.4"},
      {:edeliver, "~> 1.4"},
      {:ex_machina, "~> 2.1", only: :test}
    ]
  end

  # Aliases are shortcut or tasks specific to the current project.
  # For example, to create, migrate and run the seeds file at once:
  #
  #     $ mix ecto.setup
  #
  # See the documentation for `Mix` for more info on aliases.
  defp aliases do
    [
      "ecto.setup": ["ecto.create", "ecto.migrate", "run priv/repo/seeds.exs"],
      "ecto.reset": ["ecto.drop", "ecto.setup"]
    ]
  end
end
