defmodule Wwwtech.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  def start(_type, _args) do
    :telemetry.attach(
      "appsignal-ecto",
      [:wwwtech, :repo, :query],
      &Appsignal.Ecto.handle_event/4,
      nil
    )

    # List all child processes to be supervised
    children = [
      # Start the Ecto repository
      Wwwtech.Repo,
      # Start the endpoint when the application starts
      WwwtechWeb.Endpoint
      # Starts a worker by calling: Wwwtech.Worker.start_link(arg)
      # {Wwwtech.Worker, arg},
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: Wwwtech.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  def config_change(changed, _new, removed) do
    WwwtechWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end