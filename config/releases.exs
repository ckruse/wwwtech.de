import Config

config :wwwtech, WwwtechWeb.Endpoint,
  http: [ip: {127, 0, 0, 1}, port: System.fetch_env!("WWWTECH_PORT")],
  url: [scheme: "https", host: "wwwtech.de", port: 443],
  secret_key_base: System.fetch_env!("WWWTECH_SECRET_KEY")

config :wwwtech, storage_path: System.fetch_env!("WWWTECH_STORAGE_PATH")
config :wwwtech, cache_path: System.fetch_env!("WWWTECH_CACHE_PATH")
config :wwwtech, :environment, :prod
config :wwwtech, :keybase, System.fetch_env!("WWWTECH_KEYBASE")

config :wwwtech, Wwwtech.Repo,
  adapter: Ecto.Adapters.Postgres,
  username: System.fetch_env!("WWWTECH_DB_USER"),
  password: System.fetch_env!("WWWTECH_DB_PASSWORD"),
  database: System.fetch_env!("WWWTECH_DB_NAME"),
  pool_size: 20

config :wwwtech, Wwwtech.Mailer,
  adapter: Swoosh.Adapters.SMTP,
  relay: System.fetch_env!("WWWTECH_SMTP_SERVER"),
  port: System.fetch_env!("WWWTECH_SMTP_PORT") |> String.to_integer(),
  username: System.fetch_env!("WWWTECH_SMTP_USER"),
  password: System.fetch_env!("WWWTECH_SMTP_PASSWORD"),
  tls: :if_available,
  auth: :always,
  retries: 3

config :appsignal, :config,
  name: "WWWTech",
  push_api_key: System.fetch_env!("WWWTECH_APPSIGNAL_KEY"),
  env: :prod,
  active: true,
  otp_app: :wwwtech,
  ecto_repos: [],
  working_directory_path: "/tmp/wwwtech",
  log_path: "/tmp/wwwtech/",
  ignore_errors: ["Phoenix.Router.NoRouteError"]

config :wwwtech,
  deploy_secret: System.fetch_env!("WWWTECH_DEPLOY_SECRET"),
  deploy_script: System.fetch_env!("WWWTECH_DEPLOY_SCRIPT")

config :gh_webhook_plug, secret: System.fetch_env!("WWWTECH_DEPLOY_SECRET")
