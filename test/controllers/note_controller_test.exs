defmodule Wwwtech.NoteControllerTest do
  use Wwwtech.ConnCase

  alias Wwwtech.Note
  @valid_attrs %{author_id: 42, content: "some content", in_reply_to: "some content", posse: true}
  @invalid_attrs %{}

  test "lists all entries on index", %{conn: conn} do
    conn = get conn, note_path(conn, :index)
    assert html_response(conn, 200) =~ "Listing notes"
  end

  test "renders form for new resources", %{conn: conn} do
    conn = get conn, note_path(conn, :new)
    assert html_response(conn, 200) =~ "New note"
  end

  test "creates resource and redirects when data is valid", %{conn: conn} do
    conn = post conn, note_path(conn, :create), note: @valid_attrs
    assert redirected_to(conn) == note_path(conn, :index)
    assert Repo.get_by(Note, @valid_attrs)
  end

  test "does not create resource and renders errors when data is invalid", %{conn: conn} do
    conn = post conn, note_path(conn, :create), note: @invalid_attrs
    assert html_response(conn, 200) =~ "New note"
  end

  test "shows chosen resource", %{conn: conn} do
    note = Repo.insert! %Note{}
    conn = get conn, note_path(conn, :show, note)
    assert html_response(conn, 200) =~ "Show note"
  end

  test "renders page not found when id is nonexistent", %{conn: conn} do
    assert_error_sent 404, fn ->
      get conn, note_path(conn, :show, -1)
    end
  end

  test "renders form for editing chosen resource", %{conn: conn} do
    note = Repo.insert! %Note{}
    conn = get conn, note_path(conn, :edit, note)
    assert html_response(conn, 200) =~ "Edit note"
  end

  test "updates chosen resource and redirects when data is valid", %{conn: conn} do
    note = Repo.insert! %Note{}
    conn = put conn, note_path(conn, :update, note), note: @valid_attrs
    assert redirected_to(conn) == note_path(conn, :show, note)
    assert Repo.get_by(Note, @valid_attrs)
  end

  test "does not update chosen resource and renders errors when data is invalid", %{conn: conn} do
    note = Repo.insert! %Note{}
    conn = put conn, note_path(conn, :update, note), note: @invalid_attrs
    assert html_response(conn, 200) =~ "Edit note"
  end

  test "deletes chosen resource", %{conn: conn} do
    note = Repo.insert! %Note{}
    conn = delete conn, note_path(conn, :delete, note)
    assert redirected_to(conn) == note_path(conn, :index)
    refute Repo.get(Note, note.id)
  end
end
