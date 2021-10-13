defmodule Wwwtech.MixProject do
  use Mix.Project

  def project do
    [
      app: :wwwtech,
      version: "0.12.6",
      elixir: "~> 1.7",
      elixirc_paths: elixirc_paths(Mix.env()),
      compilers: [:gettext] ++ Mix.compilers(),
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
      mod: {Wwwtech.Application, []},
      extra_applications: [:logger, :runtime_tools, :os_mon]
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
      {:phoenix, "~> 1.6.0"},
      {:phoenix_pubsub, "~> 2.0"},
      {:phoenix_ecto, "~> 4.0"},
      {:ecto_sql, "~> 3.1"},
      {:postgrex, ">= 0.0.0"},
      {:phoenix_html, "~> 3.0"},
      {:phoenix_live_reload, "~> 1.2", only: :dev},
      {:gettext, "~> 0.18"},
      {:jason, "~> 1.0"},
      {:plug_cowboy, "~> 2.1"},
      {:argon2_elixir, "~> 2.0"},
      {:timex, "~> 3.5"},
      {:earmark, "~> 1.4"},
      {:xml_builder, "~> 2.1"},
      {:exexif, "~> 0.0.5"},
      {:mogrify, "~> 0.9.1"},
      {:microformats2, "~> 0.1"},
      {:webmentions, "~> 1.0"},
      {:floki, "~> 0.23"},
      {:swoosh, "~> 1.3"},
      {:gen_smtp, "~> 1.1"},
      {:phoenix_swoosh, "~> 0.3"},
      {:appsignal_phoenix, "~> 2.0.0"},
      {:tesla, "~> 1.3"},
      {:hackney, "~> 1.15"},
      {:gh_webhook_plug, "~> 0.0.5"}
    ]
  end

  # Aliases are shortcuts or tasks specific to the current project.
  # For example, to create, migrate and run the seeds file at once:
  #
  #     $ mix ecto.setup
  #
  # See the documentation for `Mix` for more info on aliases.
  defp aliases do
    [
      "ecto.setup": ["ecto.create", "ecto.migrate", "run priv/repo/seeds.exs"],
      "ecto.reset": ["ecto.drop", "ecto.setup"],
      test: ["ecto.create --quiet", "ecto.migrate", "test"],
      build: "cmd ./.build/build",
      deploy: "cmd ./.build/deploy"
    ]
  end
end
