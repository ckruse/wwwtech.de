defmodule Wwwtech.CacheController do
  use Wwwtech.Web, :controller
  use Wwwtech.Web, :web_controller

  alias Wwwtech.Note
  alias Wwwtech.Picture
  alias Wwwtech.Article

  @allowed_hosts [
    ~r/^https?:\/\/staticmap.openstreetmap.de/
  ]

  def show(conn, %{"url" => url}) do
    is_allowed = Enum.any?(@allowed_hosts, fn(x) -> Regex.match?(x, url) end)

    if is_allowed do
      send_cached_reply(conn, url)
    else
      conn |> put_status(403) |> text("Access forbidden")
    end
  end

  defp send_cached_reply(conn, url) do
    file_name = url2filename(url)

    if not File.exists?(file_name) do
      get_url(url, file_name)
    end

    cache_time = Timex.Date.now |> Timex.Date.add(Timex.Time.to_timestamp(360, :days))

    conn |>
      put_resp_header("expires", cache_time |> Timex.DateFormat.format!("{RFC1123}")) |>
      put_resp_header("cache-control", "public,max-age=31536000") |>
      send_file(200, file_name)
  end

  defp get_url(url, cache_file) do
    rsp = HTTPotion.get(url)
    if HTTPotion.Response.success?(rsp) do
      File.write!(cache_file, rsp.body)
    else
      raise "Error getting URL"
    end
  end

  defp url2filename(url) do
    Application.get_env(:wwwtech, :cache_path) <> "/" <> Regex.replace(~r/[^a-z0-9A-Z]/, url, "_")
  end
end
