defmodule Wwwtech.NoteTest do
  use Wwwtech.ModelCase

  alias Wwwtech.Note

  @valid_attrs %{author_id: 42, content: "some content", in_reply_to: "some content", posse: true}
  @invalid_attrs %{}

  test "changeset with valid attributes" do
    changeset = Note.changeset(%Note{}, @valid_attrs)
    assert changeset.valid?
  end

  test "changeset with invalid attributes" do
    changeset = Note.changeset(%Note{}, @invalid_attrs)
    refute changeset.valid?
  end
end
