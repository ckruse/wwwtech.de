defmodule WwwtechWeb.ArticleControllerTest do
  use WwwtechWeb.ConnCase
  import Wwwtech.Factory

  alias Wwwtech.Articles

  setup do
    {:ok, author: build(:author) |> insert}
  end

  describe "index" do
    test "lists all articles", %{conn: conn} do
      conn = get(conn, article_path(conn, :index))
      assert html_response(conn, 200) =~ "<h2>Articles</h2>"
    end
  end

  describe "show" do
    test "renders an article", %{conn: conn} do
      article = insert(:article)
      path = WwwtechWeb.ArticleView.show_article_path(conn, article)
      conn = get(conn, path)
      assert html_response(conn, 200) =~ "<h3 class=\"p-name\"><a href=\"#{path}\">#{article.title}</a></h3>"
    end
  end

  describe "new article" do
    test "renders form", %{conn: conn, author: author} do
      conn = login(conn, author) |> get(article_path(conn, :new))
      assert html_response(conn, 200) =~ "<h2>New article</h2>"
    end
  end

  describe "create article" do
    test "redirects to show when data is valid", %{conn: conn, author: author} do
      conn =
        login(conn, author)
        |> post(article_path(conn, :create), article: params_for(:article))

      assert redirected_to(conn) == article_path(conn, :index)
    end

    test "renders errors when data is invalid", %{conn: conn, author: author} do
      conn =
        login(conn, author)
        |> post(article_path(conn, :create), article: %{})

      assert html_response(conn, 200) =~ "<h2>New article</h2>"
    end
  end

  describe "edit article" do
    test "renders form for editing chosen article", %{conn: conn, author: author} do
      article = insert(:article)

      conn =
        login(conn, author)
        |> get(article_path(conn, :edit, article))

      assert html_response(conn, 200) =~ "<h2>Edit article</h2>"
    end
  end

  describe "update article" do
    test "redirects when data is valid", %{conn: conn, author: author} do
      article = insert(:article)

      conn =
        login(conn, author)
        |> put(article_path(conn, :update, article), article: %{title: "foo bar"})

      assert redirected_to(conn) == article_path(conn, :index)

      article = Articles.get_article!(article.id)
      assert article.title == "foo bar"
    end

    test "renders errors when data is invalid", %{conn: conn, author: author} do
      article = insert(:article)

      conn =
        login(conn, author)
        |> put(article_path(conn, :update, article), article: %{title: ""})

      assert html_response(conn, 200) =~ "<h2>Edit article</h2>"
    end
  end

  describe "delete article" do
    test "deletes chosen article", %{conn: conn, author: author} do
      article = insert(:article)
      path = WwwtechWeb.ArticleView.show_article_path(conn, article)

      conn =
        login(conn, author)
        |> delete(article_path(conn, :delete, article))

      assert redirected_to(conn) == article_path(conn, :index)
      assert_error_sent(404, fn -> get(conn, path) end)
    end
  end
end
