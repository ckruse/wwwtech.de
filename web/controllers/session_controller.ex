defmodule Wwwtech.SessionController do
  use Wwwtech.Web, :controller
  use Wwwtech.Web, :web_controller

  def new(conn, _params) do
    if logged_in?(conn) do
      conn
      |> put_flash(:info, "You are already logged in")
      |> redirect to: "/"
    else
      render conn, "new.html"
    end
  end

  def create(conn, %{"session" => session_params}) do
    case Wwwtech.Session.login(session_params, Wwwtech.Repo) do
      {:ok, user} ->
        conn
        |> put_session(:current_user, user.id)
        |> put_flash(:info, "Logged in")
        |> redirect(to: "/")
      :error ->
        conn
        |> put_flash(:info, "Wrong email or password")
        |> render("new.html")
    end
  end

  def delete(conn, _) do
    conn
    |> delete_session(:current_user)
    |> put_flash(:info, "Logged out")
    |> redirect(to: "/")
  end
end
