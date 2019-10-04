defmodule Wwwtech.Accounts.Session do
  alias Wwwtech.Accounts

  def login(params, _) do
    params["email"]
    |> Accounts.get_author_by_email()
    |> Argon2.check_pass(params["password"], hash_key: :encrypted_password)
  end

  def current_user(conn)

  def current_user(%{assigns: %{current_user: user}}) when not is_nil(user),
    do: user

  def current_user(conn) do
    if id = Plug.Conn.get_session(conn, :current_user),
      do: Wwwtech.Accounts.get_author!(id),
      else: nil
  end

  def logged_in?(conn), do: !!current_user(conn)
end
