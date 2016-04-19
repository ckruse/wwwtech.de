defmodule Wwwtech.PageView do
  use Wwwtech.Web, :view

  def page_title(:about, _), do: "About Christian Kruse"
  def page_title(:software, _), do: "Software"

  def page_description(:about, _), do: "About Christian Kruse"
  def page_description(:software, _), do: "Free/Libre Open Source Software by Christian Kruse"

  def entry_url(conn, entry) do
    cond do
      entry.__struct__ == Wwwtech.Picture ->
        picture_url(conn, :show, entry)
      entry.__struct__ == Wwwtech.Note ->
        note_url(conn, :show, entry)
      entry.__struct__ == Wwwtech.Article ->
        Wwwtech.ArticleView.show_article_url(conn, entry)
      entry.__struct__ == Wwwtech.Like ->
        like_url(conn, :show, entry)
      true ->
        ""
    end
  end

  def entry_html(conn, entry) do
    cond do
      entry.__struct__ == Wwwtech.Picture ->
        render(Wwwtech.PictureView, "picture.html",
               conn: conn, picture: entry, atom: true)
      entry.__struct__ == Wwwtech.Note ->
        render(Wwwtech.NoteView, "note.html",
               conn: conn, note: entry, atom: true)
      entry.__struct__ == Wwwtech.Article ->
        render(Wwwtech.ArticleView, "article.html",
          conn: conn, article: entry, atom: true)
      entry.__struct__ == Wwwtech.Like ->
        render(Wwwtech.LikeView, "like.html",
               conn: conn, like: entry, atom: true)
      true ->
        ""
    end
  end

  def entry_title(entry) do
    cond do
      entry.__struct__ == Wwwtech.Like ->
        "♥ " <> entry.in_reply_to
      true ->
        entry.title
    end
  end
end
