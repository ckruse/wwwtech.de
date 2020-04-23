# This file is responsible for configuring your application
# and its dependencies with the aid of the Mix.Config module.
#
# This configuration file is loaded before any dependency and
# is restricted to this project.

# General application configuration
use Mix.Config

config :wwwtech,
  ecto_repos: [Wwwtech.Repo]

# Configures the endpoint
config :wwwtech, WwwtechWeb.Endpoint,
  url: [host: "localhost"],
  secret_key_base: "BNJVjMMRTBj5yXCtgDkusnoTmS3H7Ez3u8q9eFp4VKysCE6SdJqAZ8snMNst68Vc",
  render_errors: [view: WwwtechWeb.ErrorView, accepts: ~w(html json)],
  pubsub_server: Wwwtech.PubSub

# Configures Elixir's Logger
config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

# Use Jason for JSON parsing in Phoenix
config :phoenix, :json_library, Jason

config :argon2_elixir, t_cost: 2, m_cost: 8

config :phoenix, :template_engines,
  eex: Appsignal.Phoenix.Template.EExEngine,
  exs: Appsignal.Phoenix.Template.ExsEngine

config :tesla, adapter: Tesla.Adapter.Hackney

# Import environment specific config. This must remain at the bottom
# of this file so it overrides the configuration defined above.
import_config "#{Mix.env()}.exs"
