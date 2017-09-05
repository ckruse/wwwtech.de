defmodule Wwwtech.LikesTest do
  use Wwwtech.DataCase
  import Wwwtech.Factory

  alias Wwwtech.Likes

  describe "likes" do
    alias Wwwtech.Likes.Like

    test "list_likes/1 returns only index likes" do
      like = insert(:like)
      insert(:like, show_in_index: false)
      assert Likes.list_likes() == [like]
    end

    test "list_likes/1 returns all likes" do
      like = insert(:like)
      invisible_like = insert(:like, show_in_index: false)
      assert Likes.list_likes(false) == [like, invisible_like]
    end

    test "count_likes/1 counts all visible likes" do
      insert(:like)
      insert(:like, show_in_index: false)
      assert Likes.count_likes() == 1
    end

    test "count_likes/1 counts all likes" do
      insert(:like)
      insert(:like, show_in_index: false)
      assert Likes.count_likes(false) == 2
    end

    test "get_like!/2 returns the like with given id" do
      like = insert(:like)
      assert Likes.get_like!(like.id) == like
    end

    test "create_like/1 with valid data creates a like" do
      author = insert(:author)
      parms = string_params_for(:like)
      assert {:ok, %Like{} = like} = Likes.create_like(author, parms)
      assert like.in_reply_to == parms["in_reply_to"]
    end

    test "create_like/1 with invalid data returns error changeset" do
      author = insert(:author)
      assert {:error, %Ecto.Changeset{}} = Likes.create_like(author, %{})
    end

    test "update_like/2 with valid data updates the like" do
      like = insert(:like)
      assert {:ok, like} = Likes.update_like(like, %{"in_reply_to" => "foo bar"})
      assert %Like{} = like
      assert like.in_reply_to == "foo bar"
    end

    test "update_like/2 with invalid data returns error changeset" do
      like = insert(:like)
      assert {:error, %Ecto.Changeset{}} = Likes.update_like(like, %{"in_reply_to" => ""})
      assert like == Likes.get_like!(like.id)
    end

    test "delete_like/1 deletes the like" do
      like = insert(:like)
      assert {:ok, %Like{}} = Likes.delete_like(like)
      assert_raise Ecto.NoResultsError, fn -> Likes.get_like!(like.id) end
    end

    test "change_like/1 returns a like changeset" do
      like = insert(:like)
      assert %Ecto.Changeset{} = Likes.change_like(like)
    end
  end
end
