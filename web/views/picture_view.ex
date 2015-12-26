defmodule Wwwtech.PictureView do
  use Wwwtech.Web, :view

  def page_title(:index, _), do: "Pictures"
  def page_title(:show, assigns), do: "Picture ##{assigns[:picture].id}"

  def page_title(:new, _), do: "New Picture"
  def page_title(:create, _), do: "New Picture"

  def page_title(:edit, _), do: "Edit Picture"
  def page_title(:update, _), do: "Edit Picture"

  def page_description(:index, _), do: "This page contains random pictures, images and impressions by Christian Kruse"
  def page_description(:show, assigns), do: assigns[:picture].title
end
