defmodule WwwtechWeb.LikeControllerTest do
  use WwwtechWeb.ConnCase
  import Wwwtech.Factory

  setup do
    {:ok, author: build(:author) |> insert}
  end

  describe "index" do
    test "lists all likes", %{conn: conn} do
      conn = get(conn, like_path(conn, :index))
      assert html_response(conn, 200) =~ "<h2>Likes</h2>"
    end
  end

  describe "show" do
    test "shows a like", %{conn: conn} do
      like = insert(:like)
      conn = get(conn, like_path(conn, :show, like.id))
      assert html_response(conn, 200) =~ "<h2>Like #{like.id}</h2>"
    end
  end

  describe "new like" do
    test "renders form", %{conn: conn, author: author} do
      conn =
        login(conn, author)
        |> get(like_path(conn, :new))

      assert html_response(conn, 200) =~ "<h2>New like</h2>"
    end
  end

  describe "create like" do
    test "redirects to show when data is valid", %{conn: conn, author: author} do
      conn =
        login(conn, author)
        |> post(like_path(conn, :create), like: params_for(:like))

      assert redirected_to(conn) == like_path(conn, :index)
    end

    test "renders errors when data is invalid", %{conn: conn, author: author} do
      conn =
        login(conn, author)
        |> post(like_path(conn, :create), like: %{})

      assert html_response(conn, 200) =~ "<h2>New like</h2>"
    end
  end

  describe "edit like" do
    test "renders form for editing chosen like", %{conn: conn, author: author} do
      like = insert(:like)

      conn =
        login(conn, author)
        |> get(like_path(conn, :edit, like))

      assert html_response(conn, 200) =~ "<h2>Edit Like #{like.id}</h2>"
    end
  end

  describe "update like" do
    test "redirects when data is valid", %{conn: conn, author: author} do
      like = insert(:like)

      conn =
        login(conn, author)
        |> put(like_path(conn, :update, like), like: %{in_reply_to: "foo bar"})

      assert redirected_to(conn) == like_path(conn, :index)

      conn = get(conn, like_path(conn, :show, like))
      assert html_response(conn, 200) =~ "foo bar"
    end

    test "renders errors when data is invalid", %{conn: conn, author: author} do
      like = insert(:like)

      conn =
        login(conn, author)
        |> put(like_path(conn, :update, like), like: %{in_reply_to: ""})

      assert html_response(conn, 200) =~ "<h2>Edit Like #{like.id}</h2>"
    end
  end

  describe "delete like" do
    test "deletes chosen like", %{conn: conn, author: author} do
      like = insert(:like)

      conn =
        login(conn, author)
        |> delete(like_path(conn, :delete, like))

      assert redirected_to(conn) == like_path(conn, :index)

      assert_error_sent(404, fn ->
        get(conn, like_path(conn, :show, like))
      end)
    end
  end
end
