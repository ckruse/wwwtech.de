defmodule WwwtechWeb.Plug.Authentication do
  import Plug.Conn
  alias Phoenix.Controller

  @login_defaults [
    flash_key: :error,
    flash_msg: "You must be logged in.",
    redirect_to: "/login"
  ]

  @logout_defaults [
    flash_key: :error,
    flash_msg: "You must be logged out.",
    redirect_to: "/"
  ]

  def require_login(conn, opts \\ []) do
    opts = Keyword.merge(@login_defaults, opts)

    case Wwwtech.Accounts.Session.logged_in?(conn) do
      true ->
        conn

      false ->
        auth_redirect(conn, opts)
    end
  end

  def require_logout(conn, opts \\ []) do
    opts = Keyword.merge(@logout_defaults, opts)

    case Wwwtech.Accounts.Session.logged_in?(conn) do
      true ->
        auth_redirect(conn, opts)

      false ->
        conn
    end
  end

  def auth_redirect(conn, opts) do
    conn
    |> Controller.put_flash(opts[:flash_key], opts[:flash_msg])
    |> Controller.redirect(to: opts[:redirect_to])
    |> halt
  end
end
