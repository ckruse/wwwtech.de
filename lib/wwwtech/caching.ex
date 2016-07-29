defmodule Wwwtech.CachingPlug do
  import Plug.Conn

  def set_caching_headers(conn, _opts \\ []) do
    if not Wwwtech.Session.logged_in?(conn) and Mix.env == :prod do
      cache_time = Timex.now |> Timex.shift(hours: 1)
      conn |>
        put_resp_header("expires", cache_time |> Timex.format!("{RFC1123}")) |>
        put_resp_header("cache-control", "public, max-age=3600")
    else
      conn
    end
  end
end
