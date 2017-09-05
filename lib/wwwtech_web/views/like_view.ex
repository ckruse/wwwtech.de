defmodule WwwtechWeb.LikeView do
  use WwwtechWeb.Web, :view

  def page_title(:index, _), do: "Likes"

  def page_title(:new, _), do: "New Like"
  def page_title(:create, _), do: "New Like"

  def page_title(:edit, _), do: "Edit Like"
  def page_title(:update, _), do: "Edit Like"

  def page_title(:show, assigns), do: "Like #" <> Integer.to_string(assigns[:like].id)

  def page_description(:index, _), do: "This page contains things I found and like"
  def page_description(:show, assigns), do: "♥ " <> assigns[:like].in_reply_to
end
