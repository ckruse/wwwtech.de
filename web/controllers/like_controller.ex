defmodule Wwwtech.LikeController do
  use Wwwtech.Web, :controller
  use Wwwtech.Web, :web_controller

  alias Wwwtech.Like

  plug :set_mention_header when action in [:index, :show]
  plug :require_login when action in [:new, :edit, :create, :update, :delete]
  plug :scrub_params, "like" when action in [:create, :update]
  plug :set_caching_headers, only: [:index, :show]

  def index(conn, params) do
    page = Like |>
      Like.sorted |>
      Like.with_author |>
      Repo.paginate(params)

    render(conn, "index.html",
      page: page,
      likes: page.entries)
  end

  def index_atom(conn, _params) do
    likes = Like |>
      Like.sorted |>
      Like.with_author |>
      Like.last_x(50) |>
      Repo.all

    render(conn, "index.atom", likes: likes)
  end


  def new(conn, _params) do
    changeset = Like.changeset(%Like{})
    render(conn, "new.html", changeset: changeset)
  end

  def create(conn, %{"like" => like_params}) do
    changeset = Like.changeset(%Like{author_id: current_user(conn).id}, like_params)

    case Repo.insert(changeset) do
      {:ok, like} ->
        urls = case Webmentions.send_webmentions(like_url(conn, :show, like)) do
                 {:ok, list} ->
                   list
                 _ ->
                   []
               end

        notice = "Like created successfully. Webmentions sent to these endpoints:\n" <> Webmentions.results_as_text(urls)

        conn
        |> put_flash(:info, notice)
        |> redirect(to: like_path(conn, :index))
      {:error, changeset} ->
        render(conn, "new.html", changeset: changeset)
    end
  end

  def show(conn, %{"id" => id}) do
    like = Like |> Like.with_author |> Repo.get!(id)
    render(conn, "show.html", like: like)
  end

  def edit(conn, %{"id" => id}) do
    like = Repo.get!(Like, id)
    changeset = Like.changeset(like)
    render(conn, "edit.html", like: like, changeset: changeset)
  end

  def update(conn, %{"id" => id, "like" => like_params}) do
    like = Repo.get!(Like, id)
    changeset = Like.changeset(like, like_params)

    case Repo.update(changeset) do
      {:ok, like} ->
        urls = case Webmentions.send_webmentions(like_url(conn, :show, like)) do
                 {:ok, list} ->
                   list
                 _ ->
                   []
               end

        notice = "Like updated successfully. Webmentions sent to these endpoints:\n" <> Webmentions.results_as_text(urls)

        conn
        |> put_flash(:info, "Like updated successfully.")
        |> redirect(to: like_path(conn, :show, like))
      {:error, changeset} ->
        render(conn, "edit.html", like: like, changeset: changeset)
    end
  end

  def delete(conn, %{"id" => id}) do
    like = Repo.get!(Like, id)

    # Here we use delete! (with a bang) because we expect
    # it to always work (and if it does not, it will raise).
    Repo.delete!(like)

    conn
    |> put_flash(:info, "Like deleted successfully.")
    |> redirect(to: like_path(conn, :index))
  end
end
