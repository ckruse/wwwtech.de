defmodule WwwtechWeb.LikeControllerTest do
  use WwwtechWeb.ConnCase

  alias Wwwtech.Likes

  @create_attrs %{in_reply_to: "some in_reply_to", posse: true, show_in_index: true}
  @update_attrs %{in_reply_to: "some updated in_reply_to", posse: false, show_in_index: false}
  @invalid_attrs %{in_reply_to: nil, posse: nil, show_in_index: nil}

  def fixture(:like) do
    {:ok, like} = Likes.create_like(@create_attrs)
    like
  end

  describe "index" do
    test "lists all likes", %{conn: conn} do
      conn = get(conn, Routes.like_path(conn, :index))
      assert html_response(conn, 200) =~ "Listing Likes"
    end
  end

  describe "new like" do
    test "renders form", %{conn: conn} do
      conn = get(conn, Routes.like_path(conn, :new))
      assert html_response(conn, 200) =~ "New Like"
    end
  end

  describe "create like" do
    test "redirects to show when data is valid", %{conn: conn} do
      conn = post(conn, Routes.like_path(conn, :create), like: @create_attrs)

      assert %{id: id} = redirected_params(conn)
      assert redirected_to(conn) == Routes.like_path(conn, :show, id)

      conn = get(conn, Routes.like_path(conn, :show, id))
      assert html_response(conn, 200) =~ "Show Like"
    end

    test "renders errors when data is invalid", %{conn: conn} do
      conn = post(conn, Routes.like_path(conn, :create), like: @invalid_attrs)
      assert html_response(conn, 200) =~ "New Like"
    end
  end

  describe "edit like" do
    setup [:create_like]

    test "renders form for editing chosen like", %{conn: conn, like: like} do
      conn = get(conn, Routes.like_path(conn, :edit, like))
      assert html_response(conn, 200) =~ "Edit Like"
    end
  end

  describe "update like" do
    setup [:create_like]

    test "redirects when data is valid", %{conn: conn, like: like} do
      conn = put(conn, Routes.like_path(conn, :update, like), like: @update_attrs)
      assert redirected_to(conn) == Routes.like_path(conn, :show, like)

      conn = get(conn, Routes.like_path(conn, :show, like))
      assert html_response(conn, 200) =~ "some updated in_reply_to"
    end

    test "renders errors when data is invalid", %{conn: conn, like: like} do
      conn = put(conn, Routes.like_path(conn, :update, like), like: @invalid_attrs)
      assert html_response(conn, 200) =~ "Edit Like"
    end
  end

  describe "delete like" do
    setup [:create_like]

    test "deletes chosen like", %{conn: conn, like: like} do
      conn = delete(conn, Routes.like_path(conn, :delete, like))
      assert redirected_to(conn) == Routes.like_path(conn, :index)
      assert_error_sent 404, fn ->
        get(conn, Routes.like_path(conn, :show, like))
      end
    end
  end

  defp create_like(_) do
    like = fixture(:like)
    {:ok, like: like}
  end
end
