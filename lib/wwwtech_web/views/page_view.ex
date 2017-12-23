defmodule WwwtechWeb.PageView do
  use WwwtechWeb.Web, :view

  def page_title(:about, _), do: "About Christian Kruse"
  def page_title(:software, _), do: "Software"
  def page_title(:more, _), do: "More"

  def page_description(:about, _), do: "About Christian Kruse"
  def page_description(:software, _), do: "Free/Libre Open Source Software by Christian Kruse"
  def page_description(:more, _), do: "More things I don't want to put into the navigation"

  def entry_url(conn, entry) do
    cond do
      entry.__struct__ == Wwwtech.Pictures.Picture ->
        picture_url(conn, :show, entry)

      entry.__struct__ == Wwwtech.Notes.Note ->
        note_url(conn, :show, entry)

      entry.__struct__ == Wwwtech.Articles.Article ->
        WwwtechWeb.ArticleView.show_article_url(conn, entry)

      entry.__struct__ == Wwwtech.Likes.Like ->
        like_url(conn, :show, entry)

      true ->
        ""
    end
  end

  def has_summary?(%Wwwtech.Articles.Article{excerpt: excerpt}) when excerpt != nil and excerpt != "", do: true
  def has_summary?(_), do: false

  def entry_html(conn, entry) do
    cond do
      entry.__struct__ == Wwwtech.Pictures.Picture ->
        render(WwwtechWeb.PictureView, "picture.html", conn: conn, picture: entry, atom: true)

      entry.__struct__ == Wwwtech.Notes.Note ->
        render(WwwtechWeb.NoteView, "note.html", conn: conn, note: entry, atom: true)

      entry.__struct__ == Wwwtech.Articles.Article ->
        render(WwwtechWeb.ArticleView, "article.html", conn: conn, article: entry, atom: true)

      entry.__struct__ == Wwwtech.Likes.Like ->
        render(Wwwtech.Likes.LikeView, "like.html", conn: conn, like: entry, atom: true)

      true ->
        ""
    end
  end

  def entry_title(entry) do
    {_, data} =
      cond do
        entry.__struct__ == Wwwtech.Likes.Like ->
          "♥ " <> entry.in_reply_to

        true ->
          entry.title
      end
      |> Phoenix.HTML.html_escape()

    data
  end
end
