defmodule Wwwtech.LayoutView do
  use Wwwtech.Web, :view

  def page_title(conn, assigns) do
    try do
      apply(view_module(conn), :page_title, [action_name(conn), assigns]) <> " — WWWTech"
    rescue
      UndefinedFunctionError -> default_page_title(conn, assigns)
      FunctionClauseError -> default_page_title(conn, assigns)
    end
  end

  def default_page_title(_conn, _assigns) do
    "WWWTech — Free/Libre Open Source Software by Christian Kruse"
  end

  def description(conn, assigns) do
    try do
      apply(view_module(conn), :page_description, [action_name(conn), assigns])
    rescue
      UndefinedFunctionError -> default_page_description(conn, assigns)
      FunctionClauseError -> default_page_description(conn, assigns)
    end
  end

  def default_page_description(conn, assigns) do
    "Personal silo (Twitter, Facebook, …) replacement of Christian Kruse"
  end
end
