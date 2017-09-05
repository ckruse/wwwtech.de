defmodule WwwtechWeb.SessionControllerTest do
  use WwwtechWeb.ConnCase
  import Wwwtech.Factory

  setup do
    {:ok, author: build(:author) |> insert}
  end

  test "new renders form", %{conn: conn} do
    conn = get conn, session_path(conn, :new)
    assert html_response(conn, 200) =~ "<h2>Login</h2>"
  end

  test "redirects to index when login data is valid", %{conn: conn, author: author} do
    conn = post(conn, session_path(conn, :create),
      session: %{"email" => author.email, "password" => "abcd"})
    assert redirected_to(conn) == "/"
  end

  test "renders form when login data is wrong", %{conn: conn, author: author} do
    conn = post(conn, session_path(conn, :create),
      session: %{"email" => author.email, "password" => "111"})
    assert html_response(conn, 200) =~ "<h2>Login</h2>"

    conn = post(conn, session_path(conn, :create),
      session: %{"email" => "foo bar", "password" => "abcd"})
    assert html_response(conn, 200) =~ "<h2>Login</h2>"
  end

  test "delete redirects to root when anonymous", %{conn: conn} do
    conn = delete(conn, session_path(conn, :delete))
    assert redirected_to(conn) == "/"
  end

  test "delete redirects to root when logged in", %{conn: conn, author: author} do
    conn = login(conn, author)
    |> delete(session_path(conn, :delete))
    assert redirected_to(conn) == "/"
  end
end
