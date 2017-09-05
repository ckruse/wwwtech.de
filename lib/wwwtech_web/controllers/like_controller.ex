defmodule WwwtechWeb.LikeController do
  use WwwtechWeb.Web, :controller
  use WwwtechWeb.Web, :web_controller

  alias WwwtechWeb.Helpers.Paging

  alias Wwwtech.Likes
  alias Wwwtech.Likes.Like

  plug :set_mention_header when action in [:index, :show]
  plug :require_login when action in [:new, :edit, :create, :update, :delete]
  plug :scrub_params, "like" when action in [:create, :update]
  plug :set_caching_headers, only: [:index, :show]

  def index(conn, params) do
    number_of_likes = Likes.count_likes(!logged_in?(conn))
    paging = Paging.paginate(number_of_likes, page: params["p"])
    likes = Likes.list_likes(!logged_in?(conn), limit: paging.params)

    render(
      conn,
      "index.html",
      paging: paging,
      likes: likes
    )
  end

  def index_atom(conn, _params) do
    likes = Likes.list_likes(true, [limit: [quantity: 10, offset: 0]])
    render(conn, "index.atom", likes: likes)
  end

  def new(conn, _params) do
    changeset = Likes.change_like(%Like{})
    render(conn, "new.html", changeset: changeset)
  end

  def create(conn, %{"like" => like_params}) do
    case Likes.create_like(current_user(conn), like_params) do
      {:ok, like} ->
        conn
        |> put_flash(:info, WwwtechWeb.Helpers.Webmentions.send_webmentions(like_url(conn, :show, like), "Like", "created"))
        |> redirect(to: like_path(conn, :index))
      {:error, %Ecto.Changeset{} = changeset} ->
        render(conn, "new.html", changeset: changeset)
    end
  end

  def show(conn, %{"id" => id}) do
    like = Likes.get_like!(id)
    render(conn, "show.html", like: like)
  end

  def edit(conn, %{"id" => id}) do
    like = Likes.get_like!(id)
    changeset = Likes.change_like(like)
    render(conn, "edit.html", like: like, changeset: changeset)
  end

  def update(conn, %{"id" => id, "like" => like_params}) do
    like = Likes.get_like!(id)

    case Likes.update_like(like, like_params) do
      {:ok, like} ->
        conn
        |> put_flash(:info, WwwtechWeb.Helpers.Webmentions.send_webmentions(like_url(conn, :show, like), "Like", "updated"))
        |> redirect(to: like_path(conn, :index))
      {:error, %Ecto.Changeset{} = changeset} ->
        render(conn, "edit.html", like: like, changeset: changeset)
    end
  end

  def delete(conn, %{"id" => id}) do
    like = Likes.get_like!(id)

    # Here we use delete! (with a bang) because we expect
    # it to always work (and if it does not, it will raise).
    Likes.delete_like(like)

    conn
    |> put_flash(:info, "Like deleted successfully.")
    |> redirect(to: like_path(conn, :index))
  end
end
