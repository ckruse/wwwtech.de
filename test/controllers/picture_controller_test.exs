defmodule Wwwtech.PictureControllerTest do
  use Wwwtech.ConnCase

  alias Wwwtech.Picture
  @valid_attrs %{author_id: 42, image_content_type: "some content", image_file_name: "some content", image_file_size: 42, image_updated_at: "2010-04-17 14:00:00", in_reply_to: "some content", posse: true, title: "some content"}
  @invalid_attrs %{}

  test "lists all entries on index", %{conn: conn} do
    conn = get conn, picture_path(conn, :index)
    assert html_response(conn, 200) =~ "Listing pictures"
  end

  test "renders form for new resources", %{conn: conn} do
    conn = get conn, picture_path(conn, :new)
    assert html_response(conn, 200) =~ "New picture"
  end

  test "creates resource and redirects when data is valid", %{conn: conn} do
    conn = post conn, picture_path(conn, :create), picture: @valid_attrs
    assert redirected_to(conn) == picture_path(conn, :index)
    assert Repo.get_by(Picture, @valid_attrs)
  end

  test "does not create resource and renders errors when data is invalid", %{conn: conn} do
    conn = post conn, picture_path(conn, :create), picture: @invalid_attrs
    assert html_response(conn, 200) =~ "New picture"
  end

  test "shows chosen resource", %{conn: conn} do
    picture = Repo.insert! %Picture{}
    conn = get conn, picture_path(conn, :show, picture)
    assert html_response(conn, 200) =~ "Show picture"
  end

  test "renders page not found when id is nonexistent", %{conn: conn} do
    assert_error_sent 404, fn ->
      get conn, picture_path(conn, :show, -1)
    end
  end

  test "renders form for editing chosen resource", %{conn: conn} do
    picture = Repo.insert! %Picture{}
    conn = get conn, picture_path(conn, :edit, picture)
    assert html_response(conn, 200) =~ "Edit picture"
  end

  test "updates chosen resource and redirects when data is valid", %{conn: conn} do
    picture = Repo.insert! %Picture{}
    conn = put conn, picture_path(conn, :update, picture), picture: @valid_attrs
    assert redirected_to(conn) == picture_path(conn, :show, picture)
    assert Repo.get_by(Picture, @valid_attrs)
  end

  test "does not update chosen resource and renders errors when data is invalid", %{conn: conn} do
    picture = Repo.insert! %Picture{}
    conn = put conn, picture_path(conn, :update, picture), picture: @invalid_attrs
    assert html_response(conn, 200) =~ "Edit picture"
  end

  test "deletes chosen resource", %{conn: conn} do
    picture = Repo.insert! %Picture{}
    conn = delete conn, picture_path(conn, :delete, picture)
    assert redirected_to(conn) == picture_path(conn, :index)
    refute Repo.get(Picture, picture.id)
  end
end
