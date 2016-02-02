defmodule Wwwtech.CachingPlug do
  import Plug.Conn

  def set_caching_headers(conn, _opts \\ []) do
    if not Wwwtech.Session.logged_in?(conn) do # and Mix.env == "prod" do
      cache_time = Timex.Date.now |> Timex.Date.add(Timex.Time.to_timestamp(1, :hours))
      conn |>
        put_resp_header("expires", cache_time |> Timex.DateFormat.format!("{RFC1123}")) |>
        put_resp_header("cache-control", "public, max-age=3600")
    else
      conn
    end
  end
end
