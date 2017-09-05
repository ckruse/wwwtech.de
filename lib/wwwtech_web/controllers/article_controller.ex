defmodule WwwtechWeb.ArticleController do
  use WwwtechWeb.Web, :controller
  use WwwtechWeb.Web, :web_controller

  alias WwwtechWeb.Helpers.Paging

  alias Wwwtech.Articles
  alias Wwwtech.Articles.Article

  plug :set_mention_header when action in [:index, :show]
  plug :require_login when action in [:new, :edit, :create, :update, :delete]
  plug :scrub_params, "article" when action in [:create, :update]
  plug :set_caching_headers, only: [:index, :show]

  def index(conn, params) do
    number_of_articles = Articles.count_articles(!logged_in?(conn))
    paging = Paging.paginate(number_of_articles, page: params["p"])
    articles = Articles.list_articles(!logged_in?(conn), limit: paging.params)
    render(conn, "index.html", paging: paging, articles: articles)
  end

  def index_atom(conn, _params) do
    articles = Articles.list_articles(true, [limit: [quantity: 10, offset: 0]])
    render(conn, "index.atom", articles: articles)
  end

  def show(conn, %{"year" => year, "mon" => mon, "slug" => slug}) do
    article = Articles.get_article_by_slug!("#{year}/#{mon}/#{slug}", !logged_in?(conn))
    render(conn, "show.html", article: article)
  end

  def new(conn, _params) do
    changeset = Articles.change_article(%Article{})
    render(conn, "new.html", changeset: changeset)
  end

  def create(conn, %{"article" => article_params}) do
    case Articles.create_article(current_user(conn), article_params) do
      {:ok, article} ->
        conn
        |> put_flash(:info, WwwtechWeb.Helpers.Webmentions.send_webmentions(article, WwwtechWeb.ArticleView.show_article_url(conn, article), "Article", "created"))
        |> redirect(to: article_path(conn, :index))
      {:error, %Ecto.Changeset{} = changeset} ->
        render(conn, "new.html", changeset: changeset)
    end
  end

  def edit(conn, %{"id" => id}) do
    article = Articles.get_article!(id, !logged_in?(conn))
    changeset = Articles.change_article(article)
    render(conn, "edit.html", article: article, changeset: changeset)
  end

  def update(conn, %{"id" => id, "article" => article_params}) do
    article = Articles.get_article!(id, !logged_in?(conn))

    case Articles.update_article(article, article_params) do
      {:ok, article} ->
        conn
        |> put_flash(:info, WwwtechWeb.Helpers.Webmentions.send_webmentions(article,
            WwwtechWeb.ArticleView.show_article_url(conn, article), "Article", "updated"))
        |> redirect(to: article_path(conn, :index))
      {:error, %Ecto.Changeset{} = changeset} ->
        render(conn, "edit.html", article: article, changeset: changeset)
    end
  end

  def delete(conn, %{"id" => id}) do
    article = Articles.get_article!(id, !logged_in?(conn))

    # Here we use delete! (with a bang) because we expect
    # it to always work (and if it does not, it will raise).
    Articles.delete_article(article)

    conn
    |> put_flash(:info, "Article deleted successfully.")
    |> redirect(to: article_path(conn, :index))
  end
end
