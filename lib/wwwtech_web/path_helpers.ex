defmodule WwwtechWeb.PathHelpers do
  alias WwwtechWeb.Router.Helpers

  def article_path(conn, :show, article), do: "#{Helpers.article_path(conn, :index)}/#{article.slug}"
  def article_url(conn, :show, article), do: "#{Helpers.article_url(conn, :index)}/#{article.slug}"
end
