defmodule WwwtechWeb.PictureController do
  require Logger

  use WwwtechWeb.Web, :controller
  use WwwtechWeb.Web, :web_controller

  alias WwwtechWeb.Helpers.Paging

  alias Wwwtech.Pictures
  alias Wwwtech.Pictures.Picture

  plug(:set_mention_header when action in [:index, :show])
  plug(:require_login when action in [:new, :edit, :create, :update, :delete])
  plug(:scrub_params, "picture" when action in [:create, :update])
  plug(:set_caching_headers, only: [:index, :show])

  def index(conn, params) do
    number_of_pictures = Pictures.count_pictures(!logged_in?(conn))
    paging = Paging.paginate(number_of_pictures, page: params["p"])
    pictures = Pictures.list_pictures(!logged_in?(conn), limit: paging.params)

    render(conn, "index.html", paging: paging, pictures: pictures)
  end

  def index_atom(conn, _params) do
    pictures = Pictures.list_pictures(!logged_in?(conn), limit: [quantity: 20, offset: 0])
    render(conn, "index.atom", pictures: pictures)
  end

  def show(conn, params) do
    {id, suffix} = parsed_id_and_suffix(params["id"])
    type = validated_type(params["type"])
    picture = Pictures.get_picture!(id)

    show_picture(suffix, conn, picture, type)
  end

  def new(conn, _params) do
    changeset = Pictures.change_picture(%Picture{})
    render(conn, "new.html", changeset: changeset)
  end

  def create(conn, %{"picture" => picture_params}) do
    case Pictures.create_picture(current_user(conn), picture_params) do
      {:ok, picture} ->
        result =
          WwwtechWeb.Helpers.Webmentions.send_webmentions(picture_url(conn, :show, picture), "Picture", "created")

        conn
        |> put_flash(:info, result)
        |> redirect(to: picture_path(conn, :index))

      {:error, %Ecto.Changeset{} = changeset} ->
        render(conn, "new.html", changeset: changeset)
    end
  end

  def edit(conn, %{"id" => id}) do
    picture = Pictures.get_picture!(id)
    changeset = Pictures.change_picture(picture)
    render(conn, "edit.html", picture: picture, changeset: changeset)
  end

  def update(conn, %{"id" => id, "picture" => picture_params}) do
    picture = Pictures.get_picture!(id)

    case Pictures.update_picture(picture, picture_params) do
      {:ok, picture} ->
        result =
          WwwtechWeb.Helpers.Webmentions.send_webmentions(picture_url(conn, :show, picture), "Picture", "updated")

        conn
        |> put_flash(:info, result)
        |> redirect(to: picture_path(conn, :show, picture))

      {:error, changeset} ->
        render(conn, "edit.html", picture: picture, changeset: changeset)
    end
  end

  def regenerate(conn, %{"id" => id}) do
    picture = Pictures.get_picture!(id)
    Pictures.generate_versions(picture)

    conn
    |> put_flash(:info, gettext("Started regenerating image versions"))
    |> redirect(to: picture_path(conn, :show, picture))
  end

  def delete(conn, %{"id" => id}) do
    picture = Pictures.get_picture!(id)

    # Here we use delete! (with a bang) because we expect
    # it to always work (and if it does not, it will raise).
    Pictures.delete_picture(picture)

    conn
    |> put_flash(:info, "Picture deleted successfully.")
    |> redirect(to: picture_path(conn, :index))
  end

  defp show_picture(nil, conn, picture, type) do
    exif_data =
      case ElixirExif.parse_file(Pictures.filename(picture, :original)) do
        {:ok, fields, _} ->
          fields

        _ ->
          %{}
      end

    render(conn, "show.html", picture: picture, type: type, exif: exif_data)
  end

  defp show_picture(_suffix, conn, picture, type) do
    {fname, do_cache} =
      case File.exists?(Pictures.filename(picture, type)) do
        true -> {Pictures.filename(picture, type), true}
        _ -> {Pictures.filename(picture, :original), false}
      end

    conn
    |> cache_headers(picture, do_cache)
    |> send_file(200, fname)
  end

  defp cache_headers(conn, picture, true) do
    cache_time = Timex.now() |> Timex.shift(days: 360)

    conn
    |> put_resp_header("content-type", picture.image_content_type)
    |> put_resp_header("expires", cache_time |> Timex.format!("{RFC1123}"))
    |> put_resp_header("cache-control", "public,max-age=31536000")
    |> put_resp_header("last-modified", Timex.format!(picture.updated_at, "{RFC1123z}"))
  end

  defp cache_headers(conn, _picture, _), do: conn

  defp validated_type("thumbnail"), do: :thumbnail
  defp validated_type("large"), do: :large
  defp validated_type(_), do: :original

  defp parsed_id_and_suffix(param) do
    case param |> String.split(".", parts: 2) |> Enum.reverse() do
      [format, id] -> {id, format}
      [_] -> {param, nil}
    end
  end
end
