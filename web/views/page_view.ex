defmodule Wwwtech.PageView do
  use Wwwtech.Web, :view

  def page_title(:about, _), do: "About Christian Kruse"
  def page_title(:software, _), do: "Software"

  def page_description(:about, _), do: "About Christian Kruse"
  def page_description(:software, _), do: "Free/Libre Open Source Software by Christian Kruse"

end
