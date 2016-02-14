defmodule Wwwtech.PictureView do
  use Wwwtech.Web, :view

  def page_title(:index, _), do: "Pictures"
  def page_title(:show, assigns), do: "Picture #{assigns[:picture].id}: #{assigns[:picture].title}"

  def page_title(:new, _), do: "New Picture"
  def page_title(:create, _), do: "New Picture"

  def page_title(:edit, _), do: "Edit Picture"
  def page_title(:update, _), do: "Edit Picture"

  def page_description(:index, _), do: "This page contains random pictures, images and impressions by Christian Kruse"
  def page_description(:show, assigns), do: assigns[:picture].title

  def to_degrees(vals, ref) do
    [d, m, s] = vals
    degrees = d + (m / 60.0) + (s / 3600.0)

    if ref != "N" and ref != "E" do
      0 - degrees
    else
      degrees
    end
  end
end
