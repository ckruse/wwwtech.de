defmodule Wwwtech.ArticleTest do
  use Wwwtech.ModelCase

  alias Wwwtech.Article

  @valid_attrs %{article_format: "some content", author_id: 42, body: "some content", excerpt: "some content", guid: "some content", in_reply_to: "some content", published: true, slug: "some content", title: "some content"}
  @invalid_attrs %{}

  test "changeset with valid attributes" do
    changeset = Article.changeset(%Article{}, @valid_attrs)
    assert changeset.valid?
  end

  test "changeset with invalid attributes" do
    changeset = Article.changeset(%Article{}, @invalid_attrs)
    refute changeset.valid?
  end
end
