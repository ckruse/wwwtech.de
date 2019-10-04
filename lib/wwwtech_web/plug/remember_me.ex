defmodule WwwtechWeb.Plug.RememberMe do
  @moduledoc """
  This plug is plugged in the browser pipeline and implements a „remember me”
  behaviour:

  - if the user is signed in it does nothing
  - if the user isn't signed in it checks if there is a `remember_me` cookie
  - if there is a `remember_me` cookie it loads the user object, signs in the
    user (sets `user_id` in session) and assigns the current_user
  """

  def init(opts), do: opts

  def call(%{assigns: %{current_user: user}} = conn) when not is_nil(user),
    do: conn

  def call(conn, _opts) do
    # do we find a cookie
    with token when not is_nil(token) and token != "" <- conn.req_cookies["remember_me"],
         {:ok, uid} <- Phoenix.Token.verify(WwwtechWeb.Endpoint, "user", token, max_age: 10 * 365 * 24 * 60 * 60) do
      current_user = Wwwtech.Accounts.get_author!(uid)
      new_token = Phoenix.Token.sign(WwwtechWeb.Endpoint, "user", current_user.id)

      conn
      |> Plug.Conn.put_session(:current_user, current_user.id)
      |> Plug.Conn.configure_session(renew: true)
      |> Plug.Conn.assign(:current_user, current_user)
      |> Plug.Conn.put_resp_cookie("remember_me", new_token, max_age: 10 * 365 * 24 * 60 * 60, http_only: true)
    else
      _ -> conn
    end
  end
end
