defmodule WwwtechWeb.Plug.Webmention do
  import Plug.Conn

  def set_mention_header(conn, _opts \\ []) do
    conn
    |> put_resp_header("link", "<#{WwwtechWeb.Router.Helpers.webmention_url(conn, :create)}>; rel=\"webmention\"")
  end
end
