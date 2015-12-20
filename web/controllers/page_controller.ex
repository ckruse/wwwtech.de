defmodule Wwwtech.PageController do
  use Wwwtech.Web, :controller

  def index(conn, _params) do
    render conn, "index.html"
  end

  def about(conn, _params) do
    render conn, "about.html"
  end

  def software(conn, _params) do
    render conn, "software.html"
  end
end
