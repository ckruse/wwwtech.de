defmodule WwwtechWeb.ArticleController do
  use WwwtechWeb, :controller

  alias Wwwtech.Articles
  alias Wwwtech.Articles.Article
  alias WwwtechWeb.Paging
  alias Wwwtech.Mentions
  alias WwwtechWeb.ArticleView

  plug :set_mention_header when action in [:index, :show]
  plug :set_caching_headers when action in [:index, :show]
  plug :require_login when action in [:new, :edit, :create, :update, :delete]

  def index(conn, params) do
    number_of_articles = Articles.count_articles(show_hidden: logged_in?(conn))
    paging = Paging.paginate(number_of_articles, page: params["p"])

    articles =
      Articles.list_articles(
        show_hidden: logged_in?(conn),
        with: [:author],
        limit: paging.limit,
        offset: paging.offset
      )

    render(conn, "index.html", articles: articles, paging: paging)
  end

  def index_atom(conn, _params) do
    articles = Articles.list_articles(limit: 10, offset: 0)

    callbacks = %{
      title: "WWWTech / Articles",
      id: Routes.article_url(conn, :index) <> ".atom",
      self_url: Routes.article_url(conn, :index) <> ".atom",
      alternate_url: Routes.article_url(conn, :index),
      entry_url: &WwwtechWeb.ArticleView.show_article_url(conn, &1),
      entry_id: &"tag:wwwtech.de,2005:Article/#{&1.id}",
      entry_title: & &1.title,
      entry_content:
        &Phoenix.View.render_to_string(WwwtechWeb.ArticleView, "article.html", article: &1, atom: true, conn: conn)
    }

    conn
    |> put_resp_content_type("application/atom+xml", "utf-8")
    |> send_resp(200, WwwtechWeb.Atom.to_atom(articles, callbacks))
  end

  def new(conn, _params) do
    changeset = Articles.change_article(%Article{})
    render(conn, "new.html", changeset: changeset)
  end

  def create(conn, %{"article" => article_params}) do
    article_params =
      article_params
      |> Map.put("author_id", conn.assigns[:current_user].id)
      |> Map.update("slug", "", &"#{Timex.format!(Timex.now(), "%Y/%b", :strftime) |> String.downcase()}/#{&1}")
      |> Map.put("guid", ArticleView.show_article_url(conn, %{slug: article_params["slug"]}))

    case Articles.create_article(article_params) do
      {:ok, article} ->
        info =
          if article.published do
            Mentions.send_webmentions(
              article,
              WwwtechWeb.ArticleView.show_article_url(conn, article),
              "Article",
              "created"
            )
          else
            "Article has successfully been created"
          end

        conn
        |> put_flash(:info, info)
        |> redirect(to: PathHelpers.article_path(conn, :show, article))

      {:error, %Ecto.Changeset{} = changeset} ->
        render(conn, "new.html", changeset: changeset)
    end
  end

  def show(conn, %{"year" => year, "mon" => mon, "slug" => slug}) do
    article =
      Articles.get_article_by_slug!("#{year}/#{mon}/#{slug}", show_hidden: logged_in?(conn), with: [:author, :mentions])

    render(conn, "show.html", article: article)
  rescue
    _e in Ecto.NoResultsError ->
      article = Articles.search_article_by_slug_part(slug, show_hidden: logged_in?(conn))

      conn
      |> put_status(301)
      |> redirect(to: PathHelpers.article_path(conn, :show, article))
  end

  def edit(conn, %{"id" => id}) do
    article = Articles.get_article!(id)
    changeset = Articles.change_article(article)
    render(conn, "edit.html", article: article, changeset: changeset)
  end

  def update(conn, %{"id" => id, "article" => article_params}) do
    article = Articles.get_article!(id)

    case Articles.update_article(article, article_params) do
      {:ok, article} ->
        info =
          if article.published do
            Mentions.send_webmentions(
              article,
              WwwtechWeb.ArticleView.show_article_url(conn, article),
              "Article",
              "updated"
            )
          else
            "Article has successfully been updated."
          end

        conn
        |> put_flash(:info, info)
        |> redirect(to: PathHelpers.article_path(conn, :show, article))

      {:error, %Ecto.Changeset{} = changeset} ->
        render(conn, "edit.html", article: article, changeset: changeset)
    end
  end

  def delete(conn, %{"id" => id}) do
    article = Articles.get_article!(id)
    {:ok, _article} = Articles.delete_article(article)

    conn
    |> put_flash(:info, "Article deleted successfully.")
    |> redirect(to: Routes.article_path(conn, :index))
  end
end
