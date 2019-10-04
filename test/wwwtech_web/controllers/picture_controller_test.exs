defmodule WwwtechWeb.PictureControllerTest do
  use WwwtechWeb.ConnCase

  alias Wwwtech.Pictures

  @create_attrs %{content: "some content", image_content_type: "some image_content_type", image_file_name: "some image_file_name", image_file_size: "some image_file_size", image_updated_at: ~N[2010-04-17 14:00:00], in_reply_to: "some in_reply_to", lang: "some lang", posse: true, show_in_index: true, title: "some title"}
  @update_attrs %{content: "some updated content", image_content_type: "some updated image_content_type", image_file_name: "some updated image_file_name", image_file_size: "some updated image_file_size", image_updated_at: ~N[2011-05-18 15:01:01], in_reply_to: "some updated in_reply_to", lang: "some updated lang", posse: false, show_in_index: false, title: "some updated title"}
  @invalid_attrs %{content: nil, image_content_type: nil, image_file_name: nil, image_file_size: nil, image_updated_at: nil, in_reply_to: nil, lang: nil, posse: nil, show_in_index: nil, title: nil}

  def fixture(:picture) do
    {:ok, picture} = Pictures.create_picture(@create_attrs)
    picture
  end

  describe "index" do
    test "lists all pictures", %{conn: conn} do
      conn = get(conn, Routes.picture_path(conn, :index))
      assert html_response(conn, 200) =~ "Listing Pictures"
    end
  end

  describe "new picture" do
    test "renders form", %{conn: conn} do
      conn = get(conn, Routes.picture_path(conn, :new))
      assert html_response(conn, 200) =~ "New Picture"
    end
  end

  describe "create picture" do
    test "redirects to show when data is valid", %{conn: conn} do
      conn = post(conn, Routes.picture_path(conn, :create), picture: @create_attrs)

      assert %{id: id} = redirected_params(conn)
      assert redirected_to(conn) == Routes.picture_path(conn, :show, id)

      conn = get(conn, Routes.picture_path(conn, :show, id))
      assert html_response(conn, 200) =~ "Show Picture"
    end

    test "renders errors when data is invalid", %{conn: conn} do
      conn = post(conn, Routes.picture_path(conn, :create), picture: @invalid_attrs)
      assert html_response(conn, 200) =~ "New Picture"
    end
  end

  describe "edit picture" do
    setup [:create_picture]

    test "renders form for editing chosen picture", %{conn: conn, picture: picture} do
      conn = get(conn, Routes.picture_path(conn, :edit, picture))
      assert html_response(conn, 200) =~ "Edit Picture"
    end
  end

  describe "update picture" do
    setup [:create_picture]

    test "redirects when data is valid", %{conn: conn, picture: picture} do
      conn = put(conn, Routes.picture_path(conn, :update, picture), picture: @update_attrs)
      assert redirected_to(conn) == Routes.picture_path(conn, :show, picture)

      conn = get(conn, Routes.picture_path(conn, :show, picture))
      assert html_response(conn, 200) =~ "some updated content"
    end

    test "renders errors when data is invalid", %{conn: conn, picture: picture} do
      conn = put(conn, Routes.picture_path(conn, :update, picture), picture: @invalid_attrs)
      assert html_response(conn, 200) =~ "Edit Picture"
    end
  end

  describe "delete picture" do
    setup [:create_picture]

    test "deletes chosen picture", %{conn: conn, picture: picture} do
      conn = delete(conn, Routes.picture_path(conn, :delete, picture))
      assert redirected_to(conn) == Routes.picture_path(conn, :index)
      assert_error_sent 404, fn ->
        get(conn, Routes.picture_path(conn, :show, picture))
      end
    end
  end

  defp create_picture(_) do
    picture = fixture(:picture)
    {:ok, picture: picture}
  end
end
