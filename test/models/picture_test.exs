defmodule Wwwtech.PictureTest do
  use Wwwtech.ModelCase

  alias Wwwtech.Picture

  @valid_attrs %{author_id: 42, image_content_type: "some content", image_file_name: "some content", image_file_size: 42, image_updated_at: "2010-04-17 14:00:00", in_reply_to: "some content", posse: true, title: "some content"}
  @invalid_attrs %{}

  test "changeset with valid attributes" do
    changeset = Picture.changeset(%Picture{}, @valid_attrs)
    assert changeset.valid?
  end

  test "changeset with invalid attributes" do
    changeset = Picture.changeset(%Picture{}, @invalid_attrs)
    refute changeset.valid?
  end
end
