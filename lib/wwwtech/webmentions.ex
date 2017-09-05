defmodule Wwwtech.WebmentionPlug do
  import Plug.Conn

  def set_mention_header(conn, _opts \\ []) do
    conn |>
      put_resp_header("link",
                      "<" <> WwwtechWeb.Router.Helpers.page_url(conn, :index) <> "webmentions>; rel=\"webmention\"")
  end
end
