defmodule WwwtechWeb.Endpoint do
  use Phoenix.Endpoint, otp_app: :wwwtech
  use Appsignal.Phoenix

  socket "/socket", WwwtechWeb.UserSocket,
    websocket: true,
    longpoll: false

  socket "/live", Phoenix.LiveView.Socket

  # Serve at "/" the static files from "priv/static" directory.
  #
  # You should set gzip to true if you are running phx.digest
  # when deploying your static files in production.
  plug Plug.Static,
    at: "/",
    from: :wwwtech,
    gzip: false,
    only: ~w(css fonts images js favicon.ico robots.txt A99A9D73.asc)

  # Code reloading can be explicitly enabled under the
  # :code_reloader configuration of your endpoint.
  if code_reloading? do
    socket "/phoenix/live_reload/socket", Phoenix.LiveReloader.Socket
    plug Phoenix.LiveReloader
    plug Phoenix.CodeReloader
  end

  plug Plug.RequestId
  plug Plug.Telemetry, event_prefix: [:phoenix, :endpoint]

  plug GhWebhookPlug,
    secret: Application.get_env(:wwwtech, :deploy_secret),
    path: "/api/deploy",
    action: {Wwwtech.DeployTask, :deploy}

  plug Plug.Parsers,
    parsers: [:urlencoded, :multipart, :json],
    pass: ["*/*"],
    json_decoder: Phoenix.json_library()

  plug Plug.MethodOverride
  plug Plug.Head

  # The session will be stored in the cookie and signed,
  # this means its contents can be read but not tampered with.
  # Set :encryption_salt if you would also like to encrypt it.
  plug Plug.Session,
    store: :cookie,
    key: "_wwwtech_key",
    signing_salt: "xT8wqXK3"

  plug WwwtechWeb.Router
end
