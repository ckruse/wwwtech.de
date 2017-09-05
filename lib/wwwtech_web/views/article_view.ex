defmodule WwwtechWeb.ArticleView do
  use WwwtechWeb.Web, :view

  def page_title(:index, _), do: "Articles"

  def page_title(:new, _), do: "New Article"
  def page_title(:create, _), do: "New Article"

  def page_title(:edit, _), do: "Edit Article"
  def page_title(:update, _), do: "New Article"

  def page_title(:show, assigns), do: assigns[:article].title <> " — Articles"

  def page_description(:index, _), do: "This page contains articles by Christian Kruse"
  def page_description(:show, assigns), do: assigns[:article].title

  def show_article_path(conn, article) do
    article_path(conn, :index) <> "/" <> article.slug
  end

  def show_article_url(conn, article) do
    article_url(conn, :index) <> "/" <> article.slug
  end
end
