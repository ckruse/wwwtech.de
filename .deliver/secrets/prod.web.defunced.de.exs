use Mix.Config

# For production, we configure the host to read the PORT
# from the system environment. Therefore, you will need
# to set PORT=80 before running your server.
#
# You should also configure the url host to something
# meaningful, we use this information when generating URLs.
#
# Finally, we also include the path to a manifest
# containing the digested version of static files. This
# manifest is generated by the mix phoenix.digest task
# which you typically run after static files are built.
config :wwwtech, WwwtechWeb.Endpoint,
  http: [ip: {127, 0, 0, 1}, port: 4000],
  url: [scheme: "https", host: "wwwtech.de", port: 443],
  force_ssl: [rewrite_on: [:x_forwarded_proto]],
  cache_static_manifest: "priv/static/cache_manifest.json",
  server: true,
  root: ".",
  version: Mix.Project.config()[:version]

# Do not print debug messages in production
config :logger, level: :info

config :wwwtech, storage_path: "/home/ckruse/.wwwtech/pictures"
config :wwwtech, cache_path: "/home/ckruse/.wwwtech/cache"
config :wwwtech, :environment, :prod
config :wwwtech, :keybase, "/home/ckruse/.wwwtech/keybase.txt"

# ## SSL Support
#
# To get SSL working, you will need to add the `https` key
# to the previous section and set your `:url` port to 443:
#
#     config :wwwtech, WwwtechWeb.Endpoint,
#       ...
#       url: [host: "example.com", port: 443],
#       https: [port: 443,
#               keyfile: System.get_env("SOME_APP_SSL_KEY_PATH"),
#               certfile: System.get_env("SOME_APP_SSL_CERT_PATH")]
#
# Where those two env variables return an absolute path to
# the key and cert in disk or a relative path inside priv,
# for example "priv/ssl/server.key".
#
# We also recommend setting `force_ssl`, ensuring no data is
# ever sent via http, always redirecting to https:
#
#     config :wwwtech, WwwtechWeb.Endpoint,
#       force_ssl: [hsts: true]
#
# Check `Plug.SSL` for all available options in `force_ssl`.

# ## Using releases
#
# If you are doing OTP releases, you need to instruct Phoenix
# to start the server for all endpoints:
#
#     config :phoenix, :serve_endpoints, true
#
# Alternatively, you can configure exactly which server to
# start per endpoint:
#
#     config :wwwtech, WwwtechWeb.Endpoint, server: true
#
# You will also need to set the application root to `.` in order
# for the new static assets to be served after a hot upgrade:
#
#     config :wwwtech, WwwtechWeb.Endpoint, root: "."

# In this file, we keep production configuration that
# you likely want to automate and keep it away from
# your version control system.
config :wwwtech, Wwwtech.Endpoint, secret_key_base: "Pxaj8ddVzTYZo1wlzw4mziYzgdQXHaYtVCCIdLqi+GQxTOdbBjbQwRZn5gjQJKCy"

# Configure your database
config :wwwtech, Wwwtech.Repo,
  adapter: Ecto.Adapters.Postgres,
  username: "postgres",
  password: "postgres",
  database: "wwwtech_prod",
  pool_size: 20

config :wwwtech, Wwwtech.Mailer,
  adapter: Bamboo.SMTPAdapter,
  server: "mail.defunced.de",
  port: 25,
  username: "cjk@defunct.ch",
  password: "WoijthoyftEbOp3",
  # can be `:always` or `:never`
  tls: :if_available,
  # can be `true`
  ssl: false,
  retries: 1
