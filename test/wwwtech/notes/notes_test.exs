defmodule Wwwtech.NotesTest do
  use Wwwtech.DataCase
  import Wwwtech.Factory

  alias Wwwtech.Notes

  describe "notes" do
    alias Wwwtech.Notes.Note

    test "list_notes/1 returns only index notes" do
      note = insert(:note)
      insert(:note, show_in_index: false)
      assert Notes.list_notes() == [note]
    end

    test "list_notes/1 returns all notes" do
      note = insert(:note)
      invisible_note = insert(:note, show_in_index: false)
      assert Notes.list_notes(false) == [note, invisible_note]
    end

    test "count_notes/1 counts all visible notes" do
      insert(:note)
      insert(:note, show_in_index: false)
      assert Notes.count_notes() == 1
    end

    test "count_notes/1 counts all notes" do
      insert(:note)
      insert(:note, show_in_index: false)
      assert Notes.count_notes(false) == 2
    end

    test "get_note!/2 returns the note with given id" do
      note = insert(:note)
      assert Notes.get_note!(note.id) == note
    end

    test "create_note/1 with valid data creates a note" do
      author = insert(:author)
      parms = string_params_for(:note)
      assert {:ok, %Note{} = note} = Notes.create_note(author, parms)
      assert note.title == parms["title"]
    end

    test "create_note/1 with invalid data returns error changeset" do
      author = insert(:author)
      assert {:error, %Ecto.Changeset{}} = Notes.create_note(author, %{})
    end

    test "update_note/2 with valid data updates the note" do
      note = insert(:note)
      assert {:ok, note} = Notes.update_note(note, %{"title" => "foo bar"})
      assert %Note{} = note
      assert note.title == "foo bar"
    end

    test "update_note/2 with invalid data returns error changeset" do
      note = insert(:note)
      assert {:error, %Ecto.Changeset{}} = Notes.update_note(note, %{"title" => ""})
      assert note == Notes.get_note!(note.id)
    end

    test "delete_note/1 deletes the note" do
      note = insert(:note)
      assert {:ok, %Note{}} = Notes.delete_note(note)
      assert_raise Ecto.NoResultsError, fn -> Notes.get_note!(note.id) end
    end

    test "change_note/1 returns a note changeset" do
      note = insert(:note)
      assert %Ecto.Changeset{} = Notes.change_note(note)
    end
  end
end
