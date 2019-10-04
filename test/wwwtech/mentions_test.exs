defmodule Wwwtech.MentionsTest do
  use Wwwtech.DataCase

  alias Wwwtech.Mentions

  describe "mentions" do
    alias Wwwtech.Mentions.Mention

    test "list_mentions/0 returns all mentions" do
      mention = insert!(:mention)
      assert Mentions.list_mentions(limit: nil) == [mention]
    end

    test "count_mentions/0 counts all mentions" do
      insert!(:mention)
      assert Mentions.count_mentions() == 1
    end

    test "get_mention!/1 returns the mention with given id" do
      mention = insert!(:mention)
      assert Mentions.get_mention!(mention.id) == mention
    end

    test "create_mention/1 with valid data creates a mention" do
      attrs = build(:mention) |> attrs()

      assert {:ok, %Mention{} = mention} = Mentions.create_mention(attrs)
      assert mention.author == attrs[:author]
      assert mention.author_avatar == attrs[:author_avatar]
      assert mention.author_url == attrs[:author_url]
      assert mention.excerpt == attrs[:excerpt]
      assert mention.mention_type == attrs[:mention_type]
      assert mention.source_url == attrs[:source_url]
      assert mention.target_url == attrs[:target_url]
      assert mention.title == attrs[:title]
    end

    test "create_mention/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Mentions.create_mention(%{})
    end

    test "update_mention/2 with valid data updates the mention" do
      mention = insert!(:mention)

      assert {:ok, %Mention{} = new_mention} = Mentions.update_mention(mention, %{author: "foo bar"})
      assert new_mention.author == "foo bar"
      assert new_mention.author_avatar == mention.author_avatar
      assert new_mention.author_url == mention.author_url
      assert new_mention.excerpt == mention.excerpt
      assert new_mention.mention_type == mention.mention_type
      assert new_mention.source_url == mention.source_url
      assert new_mention.target_url == mention.target_url
      assert new_mention.title == mention.title
    end

    test "update_mention/2 with invalid data returns error changeset" do
      mention = insert!(:mention)
      assert {:error, %Ecto.Changeset{}} = Mentions.update_mention(mention, %{author: ""})
      assert mention == Mentions.get_mention!(mention.id)
    end

    test "delete_mention/1 deletes the mention" do
      mention = insert!(:mention)
      assert {:ok, %Mention{}} = Mentions.delete_mention(mention)
      assert_raise Ecto.NoResultsError, fn -> Mentions.get_mention!(mention.id) end
    end

    test "change_mention/1 returns a mention changeset" do
      mention = insert!(:mention)
      assert %Ecto.Changeset{} = Mentions.change_mention(mention)
    end
  end
end
