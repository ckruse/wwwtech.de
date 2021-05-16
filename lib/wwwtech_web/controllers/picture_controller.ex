defmodule WwwtechWeb.PictureController do
  use WwwtechWeb, :controller

  alias Wwwtech.Pictures
  alias Wwwtech.Pictures.Picture
  alias WwwtechWeb.Paging
  alias Wwwtech.Mentions

  plug :set_mention_header when action in [:index, :show]
  plug :set_caching_headers when action in [:index, :show]
  plug :require_login when action in [:new, :edit, :create, :update, :delete]

  def index(conn, params) do
    number_of_pictures = Pictures.count_pictures(show_hidden: logged_in?(conn))
    paging = Paging.paginate(number_of_pictures, page: params["p"], per_page: 48)

    pictures =
      Pictures.list_pictures(show_hidden: logged_in?(conn), with: [:author], limit: paging.limit, offset: paging.offset)

    render(conn, "index.html", pictures: pictures, paging: paging)
  end

  def index_scrolling(conn, params) do
    number_of_pictures = Pictures.count_pictures(show_hidden: logged_in?(conn))
    paging = Paging.paginate(number_of_pictures, page: params["p"], per_page: 48)

    pictures =
      Pictures.list_pictures(show_hidden: logged_in?(conn), with: [:author], limit: paging.limit, offset: paging.offset)

    render(conn, "pictures_list.html", pictures: pictures, paging: paging, layout: false)
  end

  def index_atom(conn, _params) do
    pictures = Pictures.list_pictures(limit: 50, offset: 0)

    callbacks = %{
      title: "WWWTech / Pictures",
      id: Routes.picture_url(conn, :index) <> ".atom",
      self_url: Routes.picture_url(conn, :index) <> ".atom",
      alternate_url: Routes.picture_url(conn, :index),
      entry_url: &Routes.picture_url(conn, :show, &1),
      entry_id: &"tag:wwwtech.de,2005:Picture/#{&1.id}",
      entry_title: & &1.title,
      entry_content:
        &Phoenix.View.render_to_string(WwwtechWeb.PictureView, "picture.html", picture: &1, atom: true, conn: conn)
    }

    conn
    |> put_resp_content_type("application/atom+xml", "utf-8")
    |> send_resp(200, WwwtechWeb.Atom.to_atom(pictures, callbacks))
  end

  def new(conn, _params) do
    changeset = Pictures.change_picture(%Picture{})
    render(conn, "new.html", changeset: changeset)
  end

  def create(conn, %{"picture" => picture_params}) do
    picture_params =
      picture_params
      |> Map.put("author_id", conn.assigns[:current_user].id)
      |> put_if_blank("content", picture_params["title"])
      |> put_if_blank("title", picture_params["content"])

    sender = &Mentions.send_webmentions(Routes.picture_url(conn, :show, &1), "Picture", "created")

    case Pictures.create_picture(picture_params, sender) do
      {:ok, picture} ->
        conn
        |> put_flash(:info, "Picture created successfully")
        |> redirect(to: Routes.picture_path(conn, :show, picture))

      {:error, %Ecto.Changeset{} = changeset} ->
        render(conn, "new.html", changeset: changeset)
    end
  end

  def show(conn, %{"id" => id} = params) do
    {id, suffix} = parsed_id_and_suffix(id)
    type = validated_type(params["type"])
    picture = Pictures.get_picture!(id, with: [:author, :mentions])

    show_picture(suffix, conn, picture, type)
  end

  def edit(conn, %{"id" => id}) do
    picture = Pictures.get_picture!(id)
    changeset = Pictures.change_picture(picture)
    render(conn, "edit.html", picture: picture, changeset: changeset)
  end

  def update(conn, %{"id" => id, "picture" => picture_params}) do
    picture = Pictures.get_picture!(id)

    picture_params =
      picture_params
      |> Map.put("author_id", conn.assigns[:current_user].id)
      |> put_if_blank("content", picture_params["title"])
      |> put_if_blank("title", picture_params["content"])

    case Pictures.update_picture(picture, picture_params) do
      {:ok, picture} ->
        info = Mentions.send_webmentions(Routes.picture_url(conn, :show, picture), "Picture", "updated")

        conn
        |> put_flash(:info, info)
        |> redirect(to: Routes.picture_path(conn, :show, picture))

      {:error, %Ecto.Changeset{} = changeset} ->
        render(conn, "edit.html", picture: picture, changeset: changeset)
    end
  end

  def regenerate(conn, %{"id" => id}) do
    picture = Pictures.get_picture!(id)
    Pictures.generate_versions(picture)

    conn
    |> put_flash(:info, gettext("Started regenerating image versions"))
    |> redirect(to: Routes.picture_path(conn, :show, picture))
  end

  def delete(conn, %{"id" => id}) do
    picture = Pictures.get_picture!(id)
    {:ok, _picture} = Pictures.delete_picture(picture)

    conn
    |> put_flash(:info, "Picture deleted successfully.")
    |> redirect(to: Routes.picture_path(conn, :index))
  end

  defp show_picture(nil, conn, picture, type) do
    exif_data =
      case Exexif.exif_from_jpeg_file(Pictures.filename(picture, :original)) do
        {:ok, info} -> info
        _ -> %{}
      end

    render(conn, "show.html", picture: picture, type: type, exif: exif_data)
  end

  defp show_picture(_suffix, conn, picture, type) do
    exists = File.exists?(Pictures.filename(picture, type))

    fname =
      if exists,
        do: Pictures.filename(picture, type),
        else: Pictures.filename(picture, :original)

    conn
    |> cache_headers(picture, exists)
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

  defp cache_headers(conn, _, _), do: conn

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
