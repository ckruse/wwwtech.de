defmodule WwwtechWeb.PageController do
  use WwwtechWeb, :controller

  alias Wwwtech.Articles
  alias Wwwtech.Notes
  alias Wwwtech.Pictures
  alias Wwwtech.Likes

  alias Wwwtech.Articles.Article
  alias Wwwtech.Notes.Note
  alias Wwwtech.Pictures.Picture
  alias Wwwtech.Likes.Like

  plug :set_mention_header
  plug :set_caching_headers when action in [:index, :index_atom, :about, :software]

  def index(conn, _params) do
    {entries, article} = get_data()

    entries_by_date =
      Enum.reduce(entries, %{}, fn item, nbd ->
        date = NaiveDateTime.to_date(item.inserted_at)
        Map.update(nbd, date, [item], &(&1 ++ [item]))
      end)

    days =
      entries_by_date
      |> Map.keys()
      |> Enum.sort_by(fn d -> {d.year, d.month, d.day} end, &>=/2)

    render(conn, "index.html", article: article, entries: entries_by_date, days: days)
  end

  def index_atom(conn, _params) do
    {entries, article} = get_data()

    all_entries =
      (entries ++ [article])
      |> Enum.sort(&(Timex.compare(&1.inserted_at, &2.inserted_at) == 1))

    callbacks = %{
      title: "WWWTech / What’s new? (Combined feed)",
      id: Routes.page_url(conn, :index_atom),
      self_url: Routes.page_url(conn, :index_atom),
      alternate_url: Routes.page_url(conn, :index),
      entry_url: &entry_url(conn, &1),
      entry_id: &"tag:wwwtech.de,2005:#{id(&1)}/#{&1.id}",
      entry_title: &entry_title/1,
      entry_content: &WwwtechWeb.PageView.entry_html(&1, Map.merge(conn.assigns, %{conn: conn, atom: true}))
    }

    conn
    |> put_resp_content_type("application/atom+xml", "utf-8")
    |> send_resp(200, WwwtechWeb.Atom.to_atom(all_entries, callbacks))
  end

  def about(conn, _params) do
    render(conn, "about.html")
  end

  def software(conn, _params) do
    render(conn, "software.html")
  end

  def more(conn, _params) do
    render(conn, "more.html")
  end

  def keybase(conn, _params) do
    cache_time = Timex.now() |> Timex.shift(days: 360)
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
    article = Articles.get_last_article(with: [:author])

    entries =
      (Notes.list_notes(limit: 10, offset: 0, with: [:author]) ++
         Pictures.list_pictures(limit: 10, offset: 0, with: [:author]) ++
         Likes.list_likes(limit: 10, offset: 0, with: [:author]))
      |> Enum.sort(&(Timex.compare(&1.inserted_at, &2.inserted_at) == 1))
      |> Enum.slice(0, 10)

    {entries, article}
  end

  defp entry_url(conn, %Article{} = entry), do: WwwtechWeb.ArticleView.show_article_url(conn, entry)
  defp entry_url(conn, %Note{} = entry), do: Routes.note_url(conn, :show, entry)
  defp entry_url(conn, %Picture{} = entry), do: Routes.picture_url(conn, :show, entry)
  defp entry_url(conn, %Like{} = entry), do: Routes.like_url(conn, :show, entry)
  defp entry_url(_, _), do: ""

  defp id(%Article{}), do: "Article"
  defp id(%Note{}), do: "Note"
  defp id(%Picture{}), do: "Picture"
  defp id(%Like{}), do: "Like"
  defp id(_), do: "Whatsnew"

  def entry_title(%Like{} = entry), do: "♥ #{entry.in_reply_to}"
  def entry_title(entry), do: entry.title
end
