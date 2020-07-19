defmodule WwwtechWeb.Api.LikeController do
  use WwwtechWeb, :controller

  alias Wwwtech.Likes
  alias Wwwtech.Likes.Like

  action_fallback WwwtechWeb.FallbackController

  def index(conn, params) do
    page =
      if present?(params["p"]) && String.match?(params["p"], ~r/^\d+$/),
        do: String.to_integer(params["p"], 10),
        else: 0

    likes = Likes.list_likes(show_hidden: true, limit: 50, offset: page * 50)

    render(conn, "index.json", likes: likes)
  end

  def create(conn, %{"like" => like_params}) do
    like_params = Map.put(like_params, "author_id", conn.assigns[:current_user].id)

    with {:ok, %Like{} = like} <- Likes.create_like(like_params) do
      conn
      |> put_status(:created)
      |> put_resp_header("location", Routes.api_like_path(conn, :show, like))
      |> render("show.json", like: like)
    end
  end

  def show(conn, %{"id" => id}) do
    like = Likes.get_like!(id)
    render(conn, "show.json", like: like)
  end

  def update(conn, %{"id" => id, "like" => like_params}) do
    like = Likes.get_like!(id)
    like_params = Map.drop(like_params, ["user_id"])

    with {:ok, %Like{} = like} <- Likes.update_like(like, like_params) do
      render(conn, "show.json", like: like)
    end
  end

  def delete(conn, %{"id" => id}) do
    like = Likes.get_like!(id)

    with {:ok, %Like{}} <- Likes.delete_like(like) do
      send_resp(conn, :no_content, "")
    end
  end
end
