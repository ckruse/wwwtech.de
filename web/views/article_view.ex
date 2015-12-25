defmodule Wwwtech.ArticleView do
  use Wwwtech.Web, :view

  def page_title(:index, _), do: "Articles"

  def page_title(:new, _), do: "New Article"
  def page_title(:create, _), do: "New Article"

  def page_title(:edit, _), do: "Edit Article"
  def page_title(:update, _), do: "New Article"

  def page_title(:show, assigns), do: assigns[:article].title <> " — Articles"

  def show_article_path(conn, article) do
    article_path(conn, :index) <> "/" <> article.slug
  end

  def show_article_url(conn, article) do
    article_url(conn, :index) <> "/" <> article.slug
  end
end
