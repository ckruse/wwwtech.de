defmodule Wwwtech.Session do
  alias Wwwtech.Author

  def login(params, repo) do
    user = repo.get_by(Author, email: String.downcase(params["email"]))
    case authenticate(user, params["password"]) do
      true -> {:ok, user}
      _    -> :error
    end
  end

  defp authenticate(user, password) do
    case user do
      nil -> false
      _   -> Comeonin.Bcrypt.checkpw(password, user.encrypted_password)
    end
  end

  def current_user(conn) do
    if conn.assigns[:_user] == nil do
      id = Plug.Conn.get_session(conn, :current_user)
      if id do
        Wwwtech.Repo.get(Author, id)
      end
    else
      conn.assigns[:_user]
    end
  end

  def logged_in?(conn), do: !!current_user(conn)
end
