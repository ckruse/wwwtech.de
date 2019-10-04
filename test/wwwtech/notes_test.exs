defmodule Wwwtech.NotesTest do
  use Wwwtech.DataCase

  alias Wwwtech.Notes

  describe "notes" do
    alias Wwwtech.Notes.Note

    test "list_notes/1 returns all visible notes" do
      note = insert!(:note)
      insert!(:note, show_in_index: false)
      assert equal_objects(Notes.list_notes(limit: nil), [note])
    end

    test "list_notes/1 with `show_hidden: true` returns all notes" do
      note = insert!(:note)
      note1 = insert!(:note, show_in_index: false)

      assert equal_objects(Notes.list_notes(show_hidden: true, limit: nil), [note1, note])
    end

    test "count_notes/0 counts all visible notes" do
      insert!(:note)
      insert!(:note, show_in_index: false)
      assert Notes.count_notes() == 1
    end

    test "count_notes/1 with `show_hidden: true` counts all notes" do
      insert!(:note)
      insert!(:note, show_in_index: false)

      assert Notes.count_notes(show_hidden: true) == 2
    end

    test "get_note!/1 returns the note with given id" do
      note = insert!(:note)
      assert equal_objects(Notes.get_note!(note.id), note)
    end

    test "create_note/1 with valid data creates a note" do
      author = insert!(:author)
      attrs = build(:note) |> attrs() |> Map.put(:author_id, author.id)
      assert {:ok, %Note{} = note} = Notes.create_note(attrs)
      assert note.content == attrs[:content]
      assert note.in_reply_to == attrs[:in_reply_to]
      assert note.lang == attrs[:lang]
      assert note.note_type == attrs[:note_type]
      assert note.posse == attrs[:posse]
      assert note.show_in_index == attrs[:show_in_index]
      assert note.title == attrs[:title]
    end

    test "create_note/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Notes.create_note(%{})
    end

    test "update_note/2 with valid data updates the note" do
      note = insert!(:note)
      assert {:ok, %Note{} = new_note} = Notes.update_note(note, %{title: "Rebellion's on my mind!"})
      assert new_note.content == note.content
      assert new_note.in_reply_to == note.in_reply_to
      assert new_note.lang == note.lang
      assert new_note.note_type == note.note_type
      assert new_note.posse == note.posse
      assert new_note.show_in_index == note.show_in_index
      assert new_note.title == "Rebellion's on my mind!"
    end

    test "update_note/2 with invalid data returns error changeset" do
      note = insert!(:note)
      assert {:error, %Ecto.Changeset{}} = Notes.update_note(note, %{title: ""})
      assert equal_objects(note, Notes.get_note!(note.id))
    end

    test "delete_note/1 deletes the note" do
      note = insert!(:note)
      assert {:ok, %Note{}} = Notes.delete_note(note)
      assert_raise Ecto.NoResultsError, fn -> Notes.get_note!(note.id) end
    end

    test "change_note/1 returns a note changeset" do
      note = insert!(:note)
      assert %Ecto.Changeset{} = Notes.change_note(note)
    end
  end
end
