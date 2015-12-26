defmodule Wwwtech.PageController do
  use Wwwtech.Web, :controller

  alias Wwwtech.Note
  alias Wwwtech.Picture
  alias Wwwtech.Article

  def index(conn, _params) do
    article = Article |> Article.with_author |> Article.sorted |> Article.last_x(1) |> Repo.one
    entries = ((Note |> Note.with_author |> Note.sorted |> Note.last_x(10) |> Repo.all) ++
      (Picture |> Picture.with_author |> Picture.sorted |> Picture.last_x(10) |> Repo.all)) |>
      Enum.sort(&(Timex.Date.compare(Note.inserted_at_timex(&1), Note.inserted_at_timex(&2)) == 1)) |> Enum.slice(0, 10)

    {entries_by_day, keys} = Enum.reduce entries, {%{}, []}, fn item, {nbd, keys} ->
      {date, _} = Ecto.DateTime.to_erl(item.inserted_at)
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

  def about(conn, _params) do
    render conn, "about.html"
  end

  def software(conn, _params) do
    render conn, "software.html"
  end
end
