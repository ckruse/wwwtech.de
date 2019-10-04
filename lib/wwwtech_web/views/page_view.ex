defmodule WwwtechWeb.PageView do
  use WwwtechWeb, :view

  alias Wwwtech.Articles.Article
  alias Wwwtech.Notes.Note
  alias Wwwtech.Pictures.Picture
  alias Wwwtech.Likes.Like

  def page_title(:about, _), do: "About Christian Kruse"
  def page_title(:software, _), do: "Software"
  def page_title(:more, _), do: "More"

  def page_description(:about, _), do: "About Christian Kruse"
  def page_description(:software, _), do: "Free/Libre Open Source Software by Christian Kruse"
  def page_description(:more, _), do: "More things I don't want to put into the navigation"

  def summary?(%Article{excerpt: excerpt}) when is_present(excerpt), do: true
  def summary?(_), do: false

  def entry_html(entry, assigns)

  def entry_html(%Article{} = entry, assigns),
    do: render_to_string(WwwtechWeb.ArticleView, "article.html", Map.put(assigns, :article, entry))

  def entry_html(%Note{} = entry, assigns),
    do: render_to_string(WwwtechWeb.NoteView, "note.html", Map.put(assigns, :note, entry))

  def entry_html(%Picture{} = entry, assigns),
    do: render_to_string(WwwtechWeb.PictureView, "picture.html", Map.put(assigns, :picture, entry))

  def entry_html(%Like{} = entry, assigns),
    do: render_to_string(WwwtechWeb.LikeView, "like.html", Map.put(assigns, :like, entry))

  def entry_html(_, _, _), do: ""
end
