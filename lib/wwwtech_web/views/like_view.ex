defmodule WwwtechWeb.LikeView do
  use WwwtechWeb, :view

  def page_title(:index, _), do: "Likes"
  def page_title(:show, %{like: like}), do: "♥ #{like.in_reply_to}"
  def page_title(:new, _), do: "New like"
  def page_title(:create, _), do: "New like"
  def page_title(:edit, _), do: "Edit like"
  def page_title(:update, _), do: "Edit like"

  def page_description(:index, _), do: "Things Christian Kruse likes"
  def page_description(:show, %{like: like}), do: "♥ #{like.in_reply_to}"
end
