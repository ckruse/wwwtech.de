defmodule WwwtechWeb.LikeController do
  use WwwtechWeb, :controller

  alias Wwwtech.Likes
  alias Wwwtech.Likes.Like
  alias WwwtechWeb.Paging
  alias Wwwtech.Mentions

  plug :set_mention_header when action in [:index, :show]
  plug :set_caching_headers when action in [:index, :show]
  plug :require_login when action in [:new, :edit, :create, :update, :delete]

  def index(conn, params) do
    number_of_likes = Likes.count_likes(show_hidden: logged_in?(conn))
    paging = Paging.paginate(number_of_likes, page: params["p"])

    likes = Likes.list_likes(show_hidden: logged_in?(conn), with: [:author], limit: paging.limit, offset: paging.offset)
    render(conn, "index.html", likes: likes, paging: paging)
  end

  def new(conn, _params) do
    changeset = Likes.change_like(%Like{})
    render(conn, "new.html", changeset: changeset)
  end

  def create(conn, %{"like" => like_params}) do
    like_params =
      like_params
      |> Map.put("author_id", conn.assigns[:current_user].id)

    case Likes.create_like(like_params) do
      {:ok, like} ->
        info = Mentions.send_webmentions(Routes.like_url(conn, :show, like), "Like", "created")

        conn
        |> put_flash(:info, info)
        |> redirect(to: Routes.like_path(conn, :show, like))

      {:error, %Ecto.Changeset{} = changeset} ->
        render(conn, "new.html", changeset: changeset)
    end
  end

  def show(conn, %{"id" => id}) do
    like = Likes.get_like!(id, with: [:author])
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
        info = Mentions.send_webmentions(Routes.like_url(conn, :show, like), "Like", "updated")

        conn
        |> put_flash(:info, info)
        |> redirect(to: Routes.like_path(conn, :show, like))

      {:error, %Ecto.Changeset{} = changeset} ->
        render(conn, "edit.html", like: like, changeset: changeset)
    end
  end

  def delete(conn, %{"id" => id}) do
    like = Likes.get_like!(id)
    {:ok, _like} = Likes.delete_like(like)

    conn
    |> put_flash(:info, "Like deleted successfully.")
    |> redirect(to: Routes.like_path(conn, :index))
  end
end
