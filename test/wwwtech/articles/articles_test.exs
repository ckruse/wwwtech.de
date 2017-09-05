defmodule Wwwtech.ArticlesTest do
  use Wwwtech.DataCase
  import Wwwtech.Factory

  alias Wwwtech.Articles

  describe "articles" do
    alias Wwwtech.Articles.Article

    test "list_articles/1 returns only published articles" do
      article = insert(:article)
      insert(:article, published: false)
      assert Articles.list_articles() == [article]
    end

    test "list_articles/1 returns all articles" do
      article = insert(:article)
      invisible_article = insert(:article, published: false)
      assert Articles.list_articles(false) == [article, invisible_article]
    end

    test "count_articles/1 counts all published articles" do
      insert(:article)
      insert(:article, published: false)
      assert Articles.count_articles() == 1
    end

    test "count_articles/1 counts all articles" do
      insert(:article)
      insert(:article, published: false)
      assert Articles.count_articles(false) == 2
    end

    test "get_article!/2 returns the article with given id when published" do
      article = insert(:article)
      assert Articles.get_article!(article.id) == article
    end

    test "get_article!/2 returns the article with given id when not published and only_invisible == false" do
      article = insert(:article, published: false)
      assert Articles.get_article!(article.id, false) == article
    end

    test "get_article!/2 raises not found when not published" do
      article = insert(:article, published: false)
      assert_raise Ecto.NoResultsError, fn -> Articles.get_article!(article.id) end
    end

    test "get_article_by_slug!/2 returns the article with given slug when published" do
      article = insert(:article)
      assert Articles.get_article_by_slug!(article.slug) == article
    end

    test "get_article_by_slug!/2 returns the article with given id when not published and only_invisible == false" do
      article = insert(:article, published: false)
      assert Articles.get_article_by_slug!(article.slug, false) == article
    end

    test "get_article_by_slug!/2 raises not found when not published" do
      article = insert(:article, published: false)
      assert_raise Ecto.NoResultsError, fn -> Articles.get_article_by_slug!(article.slug) end
    end

    test "get_last_article/1 returns the last article when published" do
      insert(:article, inserted_at: Timex.shift(Timex.now, minutes: -3))
      article = insert(:article)
      assert Articles.get_last_article() == article
    end

    test "get_last_article/1 returns the last article when not published and only_invisible == false" do
      insert(:article, inserted_at: Timex.shift(Timex.now, minutes: -3))
      article = insert(:article, published: false)
      assert Articles.get_last_article(false) == article
    end

    test "get_last_article/1 ignores unpublished articles" do
      article = insert(:article)
      insert(:article, inserted_at: Timex.shift(Timex.now, minutes: +3), published: false)
      assert Articles.get_last_article() == article
    end

    test "create_article/1 with valid data creates a article" do
      author = insert(:author)
      parms = string_params_for(:article)
      assert {:ok, %Article{} = article} = Articles.create_article(author, parms)
      assert article.title == parms["title"]
    end

    test "create_article/1 with invalid data returns error changeset" do
      author = insert(:author)
      assert {:error, %Ecto.Changeset{}} =
        Articles.create_article(author, %{})
    end

    test "update_article/2 with valid data updates the article" do
      article = insert(:article)
      assert {:ok, article} = Articles.update_article(article, %{"title" => "foo bar"})
      assert %Article{} = article
      assert article.title == "foo bar"
    end

    test "update_article/2 with invalid data returns error changeset" do
      article = insert(:article)
      assert {:error, %Ecto.Changeset{}} = Articles.update_article(article, %{"title" => ""})
      assert article == Articles.get_article!(article.id)
    end

    test "delete_article/1 deletes the article" do
      article = insert(:article)
      assert {:ok, %Article{}} = Articles.delete_article(article)
      assert_raise Ecto.NoResultsError, fn -> Articles.get_article!(article.id) end
    end

    test "change_article/1 returns a article changeset" do
      article = insert(:article)
      assert %Ecto.Changeset{} = Articles.change_article(article)
    end
  end
end
