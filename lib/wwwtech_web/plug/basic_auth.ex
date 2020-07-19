defmodule WwwtechWeb.Plug.BasicAuth do
  alias Wwwtech.Accounts

  def init(opts \\ []), do: opts

  def call(conn, _opts) do
    with {user, pass} <- Plug.BasicAuth.parse_basic_auth(conn),
         {:ok, user} <- get_and_auth_user(user, pass) do
      Plug.Conn.assign(conn, :current_user, user)
    else
      _ ->
        conn
        |> Plug.BasicAuth.request_basic_auth()
        |> Plug.Conn.halt()
    end
  end

  defp get_and_auth_user(email, pass) do
    email
    |> Accounts.get_author_by_email()
    |> Argon2.check_pass(pass, hash_key: :encrypted_password)
  end
end
