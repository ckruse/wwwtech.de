defmodule Wwwtech.MentionTest do
  use Wwwtech.ModelCase

  alias Wwwtech.Mention

  @valid_attrs %{article_id: 42, author: "some content", author_avatar: "some content", author_url: "some content", excerpt: "some content", mention_type: "some content", note_id: 42, picture_id: 42, source_url: "some content", target_url: "some content", title: "some content"}
  @invalid_attrs %{}

  test "changeset with valid attributes" do
    changeset = Mention.changeset(%Mention{}, @valid_attrs)
    assert changeset.valid?
  end

  test "changeset with invalid attributes" do
    changeset = Mention.changeset(%Mention{}, @invalid_attrs)
    refute changeset.valid?
  end
end
