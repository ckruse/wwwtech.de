import Config

# Configure your database
config :wwwtech, Wwwtech.Repo,
  database: "wwwtech_test",
  hostname: "localhost",
  pool: Ecto.Adapters.SQL.Sandbox

# We don't run a server during test. If one is required,
# you can enable the server option below.
config :wwwtech, WwwtechWeb.Endpoint,
  http: [port: 4002],
  server: false

config :argon2_elixir, t_cost: 1, m_cost: 8

# Print only warnings and errors during test
config :logger, level: :warn

config :appsignal, :config, active: false
