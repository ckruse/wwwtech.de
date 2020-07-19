defmodule WwwtechWeb.Api.PictureController do
  use WwwtechWeb, :controller

  alias Wwwtech.Pictures
  alias Wwwtech.Pictures.Picture

  action_fallback WwwtechWeb.FallbackController

  def index(conn, params) do
    page =
      if present?(params["p"]) && String.match?(params["p"], ~r/^\d+$/),
        do: String.to_integer(params["p"], 10),
        else: 0

    pictures = Pictures.list_pictures(show_hidden: true, limit: 50, offset: page * 50)

    render(conn, "index.json", pictures: pictures)
  end

  def create(conn, %{"picture" => picture_params}) do
    picture_params =
      picture_params
      |> Map.put("author_id", conn.assigns[:current_user].id)
      |> put_if_blank("content", picture_params["title"])
      |> put_if_blank("title", picture_params["content"])
      |> maybe_decode_picture()

    with {:ok, %Picture{} = picture} <- Pictures.create_picture(picture_params, fn _ -> nil end) do
      conn
      |> put_status(:created)
      |> put_resp_header("location", Routes.api_picture_path(conn, :show, picture))
      |> render("show.json", picture: picture)
    end
  end

  def show(conn, %{"id" => id}) do
    picture = Pictures.get_picture!(id)
    render(conn, "show.json", picture: picture)
  end

  def update(conn, %{"id" => id, "picture" => picture_params}) do
    picture = Pictures.get_picture!(id)

    picture_params =
      picture_params
      |> Map.put("author_id", conn.assigns[:current_user].id)
      |> put_if_blank("content", picture_params["title"])
      |> put_if_blank("title", picture_params["content"])
      |> maybe_decode_picture()

    with {:ok, %Picture{} = picture} <- Pictures.update_picture(picture, picture_params) do
      render(conn, "show.json", picture: picture)
    end
  end

  def delete(conn, %{"id" => id}) do
    picture = Pictures.get_picture!(id)

    with {:ok, %Picture{}} <- Pictures.delete_picture(picture) do
      send_resp(conn, :no_content, "")
    end
  end

  defp maybe_decode_picture(%{"picture" => pic} = map) when not is_nil(pic) and pic != "",
    do: Map.put(map, "picture", {:data, Base.decode64!(pic)})

  defp maybe_decode_picture(map),
    do: Map.drop(map, ["picture"])
end
