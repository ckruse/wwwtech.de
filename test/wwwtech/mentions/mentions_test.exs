defmodule Wwwtech.MentionsTest do
  use Wwwtech.DataCase
  import Wwwtech.Factory

  alias Wwwtech.Mentions

  describe "mentions" do
    alias Wwwtech.Mentions.Mention

    test "list_mentions/1 returns all mentions" do
      mention = insert(:mention)
      assert Mentions.list_mentions() == [mention]
    end

    test "count_mentions/1 counts all mentions" do
      insert(:mention)
      assert Mentions.count_mentions() == 1
    end

    test "get_mention!/2 returns the mention with given id" do
      mention = insert(:mention)
      assert Mentions.get_mention!(mention.id) == mention
    end

    test "create_mention/1 with valid data creates a mention" do
      parms = string_params_for(:mention)
      assert {:ok, %Mention{} = mention} = Mentions.create_mention(parms)
      assert mention.source_url == parms["source_url"]
    end

    test "create_mention/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Mentions.create_mention(%{})
    end

    test "update_mention/2 with valid data updates the mention" do
      mention = insert(:mention)
      assert {:ok, mention} = Mentions.update_mention(mention, %{"source_url" => "foo bar"})
      assert %Mention{} = mention
      assert mention.source_url == "foo bar"
    end

    test "update_mention/2 with invalid data returns error changeset" do
      mention = insert(:mention)
      assert {:error, %Ecto.Changeset{}} = Mentions.update_mention(mention, %{"source_url" => ""})
      assert mention == Mentions.get_mention!(mention.id)
    end

    test "delete_mention/1 deletes the mention" do
      mention = insert(:mention)
      assert {:ok, %Mention{}} = Mentions.delete_mention(mention)
      assert_raise Ecto.NoResultsError, fn -> Mentions.get_mention!(mention.id) end
    end

    test "change_mention/1 returns a mention changeset" do
      mention = insert(:mention)
      assert %Ecto.Changeset{} = Mentions.change_mention(mention)
    end
  end
end
