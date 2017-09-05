defmodule WwwtechWeb.MentionControllerTest do
  use WwwtechWeb.ConnCase
  import Wwwtech.Factory

  setup do
    {:ok, author: build(:author) |> insert}
  end

  describe "index" do
    test "lists all mentions", %{conn: conn, author: author} do
      conn = login(conn, author)
      |> get(mention_path(conn, :index))
      assert html_response(conn, 200) =~ "<h2>Listing mentions</h2>"
    end
  end

  describe "edit mention" do
    test "renders form for editing chosen mention", %{conn: conn, author: author} do
      mention = insert(:mention)
      conn = login(conn, author)
      |> get(mention_path(conn, :edit, mention))

      assert html_response(conn, 200) =~ "<h2>Edit mention</h2>"
    end
  end

  describe "update mention" do
    test "redirects when data is valid", %{conn: conn, author: author} do
      mention = insert(:mention)
      conn = login(conn, author)
      |> put(mention_path(conn, :update, mention), mention: %{title: "foo bar"})

      assert redirected_to(conn) == mention_path(conn, :index)
    end

    test "renders errors when data is invalid", %{conn: conn, author: author} do
      mention = insert(:mention)
      conn = login(conn, author)
      |> put(mention_path(conn, :update, mention), mention: %{source_url: ""})

      assert html_response(conn, 200) =~ "<h2>Edit mention</h2>"
    end
  end

  describe "delete mention" do
    test "deletes chosen mention", %{conn: conn, author: author} do
      mention = insert(:mention)
      conn = login(conn, author)
      |> delete(mention_path(conn, :delete, mention))

      assert redirected_to(conn) == mention_path(conn, :index)
    end
  end
end
