defmodule Wwwtech.ArticleController do
  use Wwwtech.Web, :controller
  use Wwwtech.Web, :web_controller

  alias Wwwtech.Article

  plug :set_mention_header when action in [:index, :show]
  plug :require_login when action in [:new, :edit, :create, :update, :delete]
  plug :scrub_params, "article" when action in [:create, :update]
  plug :set_caching_headers, only: [:index, :show]

  def index(conn, params) do
    page = Article
    |> Article.sorted
    |> Article.with_author
    |> Article.only_visible(logged_in?(conn))
    |> Repo.paginate(page: params["page"], page_size: 10)

    render(conn, "index.html",
           page: page,
           articles: page.entries)
  end

  def index_atom(conn, _params) do
    articles = Article
    |> Article.sorted
    |> Article.with_author
    |> Article.only_visible(logged_in?(conn))
    |> Article.last_x(10)
    |> Repo.all

    render(conn, "index.atom", articles: articles)
  end

  def new(conn, _params) do
    changeset = Article.changeset(%Article{})
    render(conn, "new.html", changeset: changeset)
  end

  def create(conn, %{"article" => article_params}) do
    slug = gen_slug(article_params["slug"])
    changeset = Article.changeset(%Article{author_id: current_user(conn).id,
                                           slug: slug,
                                           guid: "https://wwwtech.de/articles/" <> slug,
                                           article_format: "markdown"}, Dict.delete(article_params, "slug"))

    case Repo.insert(changeset) do
      {:ok, article} ->
        if article.published do
          send_webmentions(Wwwtech.ArticleView.show_article_url(conn, article))
        end

        conn
        |> put_flash(:info, "Article created successfully.")
        |> redirect(to: article_path(conn, :index))
      {:error, changeset} ->
        render(conn, "new.html", changeset: changeset)
    end
  end

  def show(conn, %{"year" => year, "mon" => mon, "slug" => slug}) do
    article = Article
    |> Article.with_author
    |> Article.with_mentions
    |> Article.by_slug("#{year}/#{mon}/#{slug}")
    |> Article.only_visible(logged_in?(conn))
    |> Repo.one!

    render(conn, "show.html", article: article)
  end

  def edit(conn, %{"id" => id}) do
    article = Repo.get!(Article, id)
    changeset = Article.changeset(article)
    render(conn, "edit.html", article: article, changeset: changeset)
  end

  def update(conn, %{"id" => id, "article" => article_params}) do
    article = Repo.get!(Article, id)
    changeset = Article.changeset(article, article_params)

    case Repo.update(changeset) do
      {:ok, article} ->
        if article.published do
          send_webmentions(Wwwtech.ArticleView.show_article_url(conn, article))
        end

        conn
        |> put_flash(:info, "Article updated successfully.")
        |> redirect(to: article_path(conn, :index))
      {:error, changeset} ->
        render(conn, "edit.html", article: article, changeset: changeset)
    end
  end

  def delete(conn, %{"id" => id}) do
    article = Repo.get!(Article, id)

    # Here we use delete! (with a bang) because we expect
    # it to always work (and if it does not, it will raise).
    Repo.delete!(article)

    conn
    |> put_flash(:info, "Article deleted successfully.")
    |> redirect(to: article_path(conn, :index))
  end

  def gen_slug(slug) do
    {{year, mon, _}, _} = :calendar.local_time()
    months = [nil, "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"]
    "#{year}/#{Enum.at(months, mon)}/#{slug}"
  end
end
