defmodule Wwwtech.PageController do
  use Wwwtech.Web, :controller
  use Wwwtech.Web, :web_controller

  alias Wwwtech.Note
  alias Wwwtech.Picture
  alias Wwwtech.Article
  alias Wwwtech.Like

  plug :set_mention_header
  plug :set_caching_headers, only: [:index, :index_atom, :about, :software]

  def index(conn, _params) do
    {entries, article} = get_data()

    {entries_by_day, keys} = Enum.reduce entries, {%{}, []}, fn item, {nbd, keys} ->
      {date, _} = Timex.to_erl(item.inserted_at)
      if nbd[date] == nil do
        {Map.put(nbd, date, [item]), keys ++ [date]}
      else
        {Map.put(nbd, date, nbd[date] ++ [item]), keys}
      end
    end

    render(conn, "index.html", entries: entries,
           entries_by_day: entries_by_day, days: keys,
           article: article)
  end

  def index_atom(conn, _params) do
    {entries, article} = get_data()
    all_entries = (entries ++ [article]) |>
      Enum.sort(&(Timex.compare(&1.inserted_at, &2.inserted_at) == 1))

    render(conn, "index.atom", entries: all_entries)
  end

  def about(conn, _params) do
    render conn, "about.html"
  end

  def software(conn, _params) do
    render conn, "software.html"
  end

  def more(conn, _params) do
    render conn, "more.html"
  end


  def keybase(conn, _params) do
    cache_time = Timex.now |> Timex.shift(days: 360)
    fname = Application.get_env(:wwwtech, :keybase)

    case File.stat(fname) do
      {:ok, rec} ->
        conn
        |> put_resp_header("content-type", "text/plain; charset=uft-8")
        |> put_resp_header("last-modified", Timex.to_datetime(rec.mtime) |> Timex.format!("{RFC1123}"))
        |> put_resp_header("expires", cache_time |> Timex.format!("{RFC1123}"))
        |> put_resp_header("cache-control", "public,max-age=31536000")
        |> send_file(200, fname)

      _ ->
        conn |> put_status(404) |> text("Error: keybase.txt not found")
    end
  end

  def get_data do
    article = Article |> Article.with_author |> Article.sorted |> Article.last_x(1) |> Repo.one
    entries = ((Note |> Note.only_index(false) |> Note.with_author |> Note.sorted |> Note.last_x(10) |> Repo.all) ++
      (Picture |> Picture.with_author |> Picture.only_index(false) |> Picture.sorted |> Picture.last_x(10) |> Repo.all) ++
      (Like |> Like.with_author |> Like.sorted |> Like.last_x(10) |> Repo.all)) |>
      Enum.sort(&(Timex.compare(&1.inserted_at, &2.inserted_at) == 1)) |> Enum.slice(0, 10)

    {entries, article}
  end
end
