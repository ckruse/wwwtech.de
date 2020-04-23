defmodule WwwtechWeb.Plug.LoggedIn do
  @moduledoc """
  This plug is plugged in the browser pipeline and loads and assigns the
  current user
  """

  @opts [flash_key: :error, flash_msg: "You must be logged in.", redirect_to: "/login"]

  def init(opts), do: opts

  def call(conn, _opts) do
    if Wwwtech.Accounts.Session.logged_in?(conn),
      do: conn,
      else: WwwtechWeb.Plug.Authentication.auth_redirect(conn, @opts)
  end
end
