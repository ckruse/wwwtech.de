defmodule Wwwtech.LikesTest do
  use Wwwtech.DataCase

  alias Wwwtech.Likes

  describe "likes" do
    alias Wwwtech.Likes.Like

    test "list_likes/0 returns all visible likes" do
      like = insert!(:like, show_in_index: true)
      insert!(:like, show_in_index: false)

      assert equal_objects(Likes.list_likes(limit: nil), [like])
    end

    test "list_likes/1 with `show_hidden: true` returns all likes" do
      like = insert!(:like, show_in_index: true)
      like1 = insert!(:like, show_in_index: false)

      assert equal_objects(Likes.list_likes(show_hidden: true, limit: nil), [like1, like])
    end

    test "count_likes/0 counts all visible likes" do
      insert!(:like, show_in_index: true)
      insert!(:like, show_in_index: false)

      assert Likes.count_likes() == 1
    end

    test "count_likes/1 with `show_hidden: true` counts all likes" do
      insert!(:like, show_in_index: true)
      insert!(:like, show_in_index: false)

      assert Likes.count_likes(show_hidden: true) == 2
    end

    test "get_like!/1 returns the like with given id" do
      like = insert!(:like)
      assert equal_objects(Likes.get_like!(like.id), like)
    end

    test "create_like/1 with valid data creates a like" do
      author = insert!(:author)
      attrs = build(:like) |> attrs() |> Map.put(:author_id, author.id)

      assert {:ok, %Like{} = like} = Likes.create_like(attrs)
      assert like.in_reply_to == attrs[:in_reply_to]
      assert like.posse == attrs[:posse]
      assert like.show_in_index == attrs[:show_in_index]
    end

    test "create_like/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Likes.create_like(%{})
    end

    test "update_like/2 with valid data updates the like" do
      like = insert!(:like)
      assert {:ok, %Like{} = new_like} = Likes.update_like(like, %{in_reply_to: "http://example.org/foo"})
      assert new_like.in_reply_to == "http://example.org/foo"
      assert new_like.posse == like.posse
      assert new_like.show_in_index == like.show_in_index
    end

    test "update_like/2 with invalid data returns error changeset" do
      like = insert!(:like)
      assert {:error, %Ecto.Changeset{}} = Likes.update_like(like, %{in_reply_to: ""})
      assert equal_objects(like, Likes.get_like!(like.id))
    end

    test "delete_like/1 deletes the like" do
      like = insert!(:like)
      assert {:ok, %Like{}} = Likes.delete_like(like)
      assert_raise Ecto.NoResultsError, fn -> Likes.get_like!(like.id) end
    end

    test "change_like/1 returns a like changeset" do
      like = insert!(:like)
      assert %Ecto.Changeset{} = Likes.change_like(like)
    end
  end
end
