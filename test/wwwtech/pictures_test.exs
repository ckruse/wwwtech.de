defmodule Wwwtech.PicturesTest do
  use Wwwtech.DataCase

  alias Wwwtech.Pictures

  describe "pictures" do
    alias Wwwtech.Pictures.Picture

    test "list_pictures/0 returns all visible pictures" do
      picture = insert!(:picture, show_in_index: true)
      insert!(:picture, show_in_index: false)

      assert equal_objects(Pictures.list_pictures(limit: nil), [picture])
    end

    test "list_pictures/1 with `show_hidden: true` returns all pictures" do
      picture = insert!(:picture, show_in_index: true)
      picture1 = insert!(:picture, show_in_index: false)

      assert equal_objects(Pictures.list_pictures(limit: nil, show_hidden: true), [picture1, picture])
    end

    test "count_pictures/0 counts all visible pictures" do
      insert!(:picture, show_in_index: true)
      insert!(:picture, show_in_index: false)

      assert Pictures.count_pictures() == 1
    end

    test "count_pictures/1 with `show_hidden: true` counts all pictures" do
      insert!(:picture, show_in_index: true)
      insert!(:picture, show_in_index: false)

      assert Pictures.count_pictures(show_hidden: true) == 2
    end

    test "get_picture!/1 returns the picture with given id" do
      picture = insert!(:picture)
      assert equal_objects(Pictures.get_picture!(picture.id), picture)
    end

    test "create_picture/1 with valid data creates a picture" do
      author = insert!(:author)
      attrs = build(:picture) |> attrs() |> Map.put(:author_id, author.id)

      assert {:ok, %Picture{} = picture} = Pictures.create_picture(attrs)
      assert picture.content == attrs[:content]
      assert picture.image_content_type == attrs[:image_content_type]
      assert picture.image_file_name == attrs[:image_file_name]
      assert picture.image_file_size == attrs[:image_file_size]
      assert picture.image_updated_at == attrs[:image_updated_at]
      assert picture.in_reply_to == attrs[:in_reply_to]
      assert picture.lang == attrs[:lang]
      assert picture.posse == attrs[:posse]
      assert picture.show_in_index == attrs[:show_in_index]
      assert picture.title == attrs[:title]
    end

    test "create_picture/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Pictures.create_picture(%{})
    end

    test "update_picture/2 with valid data updates the picture" do
      picture = insert!(:picture)

      assert {:ok, %Picture{} = new_picture} = Pictures.update_picture(picture, %{title: "Freedom's calling"})
      assert new_picture.content == picture.content
      assert new_picture.image_content_type == picture.image_content_type
      assert new_picture.image_file_name == picture.image_file_name
      assert new_picture.image_file_size == picture.image_file_size
      assert new_picture.image_updated_at == picture.image_updated_at
      assert new_picture.in_reply_to == picture.in_reply_to
      assert new_picture.lang == picture.lang
      assert new_picture.posse == picture.posse
      assert new_picture.show_in_index == picture.show_in_index
      assert new_picture.title == "Freedom's calling"
    end

    test "update_picture/2 with invalid data returns error changeset" do
      picture = insert!(:picture)
      assert {:error, %Ecto.Changeset{}} = Pictures.update_picture(picture, %{title: ""})
      assert equal_objects(picture, Pictures.get_picture!(picture.id))
    end

    test "delete_picture/1 deletes the picture" do
      picture = insert!(:picture)
      assert {:ok, %Picture{}} = Pictures.delete_picture(picture)
      assert_raise Ecto.NoResultsError, fn -> Pictures.get_picture!(picture.id) end
    end

    test "change_picture/1 returns a picture changeset" do
      picture = insert!(:picture)
      assert %Ecto.Changeset{} = Pictures.change_picture(picture)
    end
  end
end
