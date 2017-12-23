defmodule Wwwtech.Accounts.Session do
  def login(params, _) do
    user = Wwwtech.Accounts.get_author_by_email(params["email"])

    case authenticate(user, params["password"]) do
      true -> {:ok, user}
      _ -> :error
    end
  end

  defp authenticate(user, password) do
    case user do
      nil -> false
      _ -> Comeonin.Bcrypt.checkpw(password, user.encrypted_password)
    end
  end

  def current_user(conn) do
    if conn.assigns[:_user] == nil do
      id = Plug.Conn.get_session(conn, :current_user)

      if id do
        Wwwtech.Accounts.get_author!(id)
      end
    else
      conn.assigns[:_user]
    end
  end

  def logged_in?(conn), do: !!current_user(conn)
end
