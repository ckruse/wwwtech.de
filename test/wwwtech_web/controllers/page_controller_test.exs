defmodule WwwtechWeb.SessionControllerTest do
  use WwwtechWeb.ConnCase
  import Wwwtech.Factory

  test "shows the main page", %{conn: conn} do
    conn = get(conn, page_path(conn, :index))
    assert html_response(conn, 200) =~ "Welcome. On this page you can find my"
  end

  test "shows the index atom", %{conn: conn} do
    insert(:article)
    insert(:note)

    conn = get(conn, page_path(conn, :index) <> "whatsnew.atom")
    assert response_content_type(conn, :atom) =~ "application/atom+xml"
  end

  test "shows about page", %{conn: conn} do
    conn = get(conn, page_path(conn, :about))

    assert html_response(conn, 200) =~
             ~r{<h2><a href="http://wwwtech.de/about" class="p-name u-url">Christian Kruse</a></h2>}
  end

  test "shows software page", %{conn: conn} do
    conn = get(conn, page_path(conn, :software))
    assert html_response(conn, 200) =~ "<h2>Software Projects</h2>"
  end

  test "shows more page", %{conn: conn} do
    conn = get(conn, page_path(conn, :more))
    assert html_response(conn, 200) =~ "<h2>More…</h2>"
  end
end
