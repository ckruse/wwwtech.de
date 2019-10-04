defmodule Wwwtech.ArticlesTest do
  use Wwwtech.DataCase

  alias Wwwtech.Articles

  describe "articles" do
    alias Wwwtech.Articles.Article

    test "list_articles/0 returns all visible articles" do
      article = insert!(:article, published: true)
      insert!(:article, published: false)

      assert equal_objects(Articles.list_articles(limit: nil), [article])
    end

    test "list_articles/1 with `show_invisible: true` returns all articles" do
      article = insert!(:article, published: true)
      article1 = insert!(:article, published: false)

      assert equal_objects(Articles.list_articles(limit: nil, show_hidden: true), [article1, article])
    end

    test "count_articles/0 counts all visible articles" do
      insert!(:article, published: true)
      insert!(:article, published: false)

      assert Articles.count_articles() == 1
    end

    test "count_articles/1 with `show_invisible: true` counts all articles" do
      insert!(:article, published: true)
      insert!(:article, published: false)

      assert Articles.count_articles(show_hidden: true) == 2
    end

    test "get_article!/1 returns the article with given id" do
      article = insert!(:article)
      assert equal_objects(Articles.get_article!(article.id), article)
    end

    test "create_article/1 with valid data creates a article" do
      author = insert!(:author)
      attrs = build(:article) |> attrs() |> Map.put(:author_id, author.id)

      assert {:ok, %Article{} = article} = Articles.create_article(attrs)
      assert article.article_format == "markdown"
      assert article.body == attrs[:body]
      assert article.excerpt == attrs[:excerpt]
      assert article.guid == attrs[:guid]
      assert article.in_reply_to == attrs[:in_reply_to]
      assert article.lang == attrs[:lang]
      assert article.posse == attrs[:posse]
      assert article.published == attrs[:published]
      assert article.slug == attrs[:slug]
      assert article.title == attrs[:title]
    end

    test "create_article/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Articles.create_article(%{})
    end

    test "update_article/2 with valid data updates the article" do
      article = insert!(:article)

      assert {:ok, %Article{} = new_article} = Articles.update_article(article, %{title: "The clans are marching"})
      assert new_article.article_format == article.article_format
      assert new_article.body == article.body
      assert new_article.excerpt == article.excerpt
      assert new_article.guid == article.guid
      assert new_article.in_reply_to == article.in_reply_to
      assert new_article.lang == article.lang
      assert new_article.posse == article.posse
      assert new_article.published == article.published
      assert new_article.slug == article.slug
      assert new_article.title == "The clans are marching"
    end

    test "update_article/2 with invalid data returns error changeset" do
      article = insert!(:article)
      assert {:error, %Ecto.Changeset{}} = Articles.update_article(article, %{title: ""})
      assert equal_objects(article, Articles.get_article!(article.id))
    end

    test "delete_article/1 deletes the article" do
      article = insert!(:article)
      assert {:ok, %Article{}} = Articles.delete_article(article)
      assert_raise Ecto.NoResultsError, fn -> Articles.get_article!(article.id) end
    end

    test "change_article/1 returns a article changeset" do
      article = insert!(:article)
      assert %Ecto.Changeset{} = Articles.change_article(article)
    end
  end
end
