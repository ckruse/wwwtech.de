defmodule Wwwtech.AuthorTest do
  use Wwwtech.ModelCase

  alias Wwwtech.Author

  @valid_attrs %{avatar: "some content", email: "some content", name: "some content"}
  @invalid_attrs %{}

  test "changeset with valid attributes" do
    changeset = Author.changeset(%Author{}, @valid_attrs)
    assert changeset.valid?
  end

  test "changeset with invalid attributes" do
    changeset = Author.changeset(%Author{}, @invalid_attrs)
    refute changeset.valid?
  end
end
