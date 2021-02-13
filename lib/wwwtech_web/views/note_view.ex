defmodule WwwtechWeb.NoteView do
  use WwwtechWeb, :view

  def page_title(:index, _), do: "Notes"

  def page_title(:new, _), do: "New Note"
  def page_title(:create, _), do: "New Note"

  def page_title(:edit, _), do: "Edit Note"
  def page_title(:update, _), do: "Edit Note"

  def page_title(:show, assigns), do: assigns[:note].title <> " — Note #" <> Integer.to_string(assigns[:note].id)

  def page_description(:index, _), do: "Random thoughts and impressions by Christian Kruse"
  def page_description(:show, assigns), do: assigns[:note].title
end
