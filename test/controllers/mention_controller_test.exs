defmodule Wwwtech.MentionControllerTest do
  use Wwwtech.ConnCase

  alias Wwwtech.Mention
  @valid_attrs %{article_id: 42, author: "some content", author_avatar: "some content", author_url: "some content", excerpt: "some content", mention_type: "some content", note_id: 42, picture_id: 42, source_url: "some content", target_url: "some content", title: "some content"}
  @invalid_attrs %{}

  test "lists all entries on index", %{conn: conn} do
    conn = get conn, mention_path(conn, :index)
    assert html_response(conn, 200) =~ "Listing mentions"
  end

  test "renders form for new resources", %{conn: conn} do
    conn = get conn, mention_path(conn, :new)
    assert html_response(conn, 200) =~ "New mention"
  end

  test "creates resource and redirects when data is valid", %{conn: conn} do
    conn = post conn, mention_path(conn, :create), mention: @valid_attrs
    assert redirected_to(conn) == mention_path(conn, :index)
    assert Repo.get_by(Mention, @valid_attrs)
  end

  test "does not create resource and renders errors when data is invalid", %{conn: conn} do
    conn = post conn, mention_path(conn, :create), mention: @invalid_attrs
    assert html_response(conn, 200) =~ "New mention"
  end

  test "shows chosen resource", %{conn: conn} do
    mention = Repo.insert! %Mention{}
    conn = get conn, mention_path(conn, :show, mention)
    assert html_response(conn, 200) =~ "Show mention"
  end

  test "renders page not found when id is nonexistent", %{conn: conn} do
    assert_error_sent 404, fn ->
      get conn, mention_path(conn, :show, -1)
    end
  end

  test "renders form for editing chosen resource", %{conn: conn} do
    mention = Repo.insert! %Mention{}
    conn = get conn, mention_path(conn, :edit, mention)
    assert html_response(conn, 200) =~ "Edit mention"
  end

  test "updates chosen resource and redirects when data is valid", %{conn: conn} do
    mention = Repo.insert! %Mention{}
    conn = put conn, mention_path(conn, :update, mention), mention: @valid_attrs
    assert redirected_to(conn) == mention_path(conn, :show, mention)
    assert Repo.get_by(Mention, @valid_attrs)
  end

  test "does not update chosen resource and renders errors when data is invalid", %{conn: conn} do
    mention = Repo.insert! %Mention{}
    conn = put conn, mention_path(conn, :update, mention), mention: @invalid_attrs
    assert html_response(conn, 200) =~ "Edit mention"
  end

  test "deletes chosen resource", %{conn: conn} do
    mention = Repo.insert! %Mention{}
    conn = delete conn, mention_path(conn, :delete, mention)
    assert redirected_to(conn) == mention_path(conn, :index)
    refute Repo.get(Mention, mention.id)
  end
end
