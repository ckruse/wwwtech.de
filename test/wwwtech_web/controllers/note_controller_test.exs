defmodule WwwtechWeb.NoteControllerTest do
  use WwwtechWeb.ConnCase
  import Wwwtech.Factory

  setup do
    {:ok, author: build(:author) |> insert}
  end

  describe "index" do
    test "lists all notes", %{conn: conn} do
      conn = get(conn, note_path(conn, :index))
      assert html_response(conn, 200) =~ "<h2>Notes</h2>"
    end
  end

  describe "show" do
    test "shows a note", %{conn: conn} do
      note = insert(:note)
      conn = get(conn, note_path(conn, :show, note.id))
      assert html_response(conn, 200) =~ "<h2>Note #{note.id}</h2>"
    end
  end

  describe "new note" do
    test "renders form", %{conn: conn, author: author} do
      conn =
        login(conn, author)
        |> get(note_path(conn, :new))

      assert html_response(conn, 200) =~ "<h2>New note</h2>"
    end
  end

  describe "create note" do
    test "redirects to show when data is valid", %{conn: conn, author: author} do
      conn =
        login(conn, author)
        |> post(note_path(conn, :create), note: params_for(:note))

      assert redirected_to(conn) == note_path(conn, :index)
    end

    test "renders errors when data is invalid", %{conn: conn, author: author} do
      conn =
        login(conn, author)
        |> post(note_path(conn, :create), note: %{})

      assert html_response(conn, 200) =~ "<h2>New note</h2>"
    end
  end

  describe "edit note" do
    test "renders form for editing chosen note", %{conn: conn, author: author} do
      note = insert(:note)

      conn =
        login(conn, author)
        |> get(note_path(conn, :edit, note))

      assert html_response(conn, 200) =~ "<h2>Edit note</h2>"
    end
  end

  describe "update note" do
    test "redirects when data is valid", %{conn: conn, author: author} do
      note = insert(:note)

      conn =
        login(conn, author)
        |> put(note_path(conn, :update, note), note: %{title: "foo bar"})

      assert redirected_to(conn) == note_path(conn, :index)

      conn = get(conn, note_path(conn, :show, note))
      assert html_response(conn, 200) =~ "foo bar"
    end

    test "renders errors when data is invalid", %{conn: conn, author: author} do
      note = insert(:note)

      conn =
        login(conn, author)
        |> put(note_path(conn, :update, note), note: %{title: ""})

      assert html_response(conn, 200) =~ "<h2>Edit note</h2>"
    end
  end

  describe "delete note" do
    test "deletes chosen note", %{conn: conn, author: author} do
      note = insert(:note)

      conn =
        login(conn, author)
        |> delete(note_path(conn, :delete, note))

      assert redirected_to(conn) == note_path(conn, :index)

      assert_error_sent(404, fn ->
        get(conn, note_path(conn, :show, note))
      end)
    end
  end
end
