defmodule Wwwtech.AuthenticationPlug do
  import Plug.Conn
  alias Phoenix.Controller

  @login_defaults [
    flash_key: :info,
    flash_msg: "You must be logged in.",
    redirect_to: "/login"
  ]

  @logout_defaults [
    flash_key: :info,
    flash_msg: "You must be logged in.",
    redirect_to: "/login"
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

  defp auth_redirect(conn, opts) do
    conn
    |> Controller.put_flash(opts[:flash_key], opts[:flash_msg])
    |> Controller.redirect(to: opts[:redirect_to])
    |> halt
  end

  def store_user(conn, _params) do
    id = Plug.Conn.get_session(conn, :current_user)

    if id do
      Plug.Conn.assign(conn, :_user, Wwwtech.Accounts.get_author!(id))
    else
      conn
    end
  end
end
