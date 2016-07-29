defmodule Wwwtech.PictureController do
  require Logger

  use Wwwtech.Web, :controller
  use Wwwtech.Web, :web_controller

  alias Wwwtech.Picture

  plug :set_mention_header when action in [:index, :show]
  plug :require_login when action in [:new, :edit, :create, :update, :delete]
  plug :scrub_params, "picture" when action in [:create, :update]
  plug :set_caching_headers, only: [:index, :show]

  def index(conn, _params) do
    pictures = Picture
    |> Picture.only_index(logged_in?(conn))
    |> Picture.sorted
    |> Picture.with_author
    |> Repo.all

    render(conn, "index.html",
           pictures: pictures)
  end

  def index_atom(conn, _params) do
    pictures = Picture
    |> Picture.sorted
    |> Picture.with_author
    |> Picture.last_x(20)
    |> Repo.all

    render(conn, "index.atom", pictures: pictures)
  end

  def new(conn, _params) do
    changeset = Picture.changeset(%Picture{})
    render(conn, "new.html", changeset: changeset)
  end

  def create(conn, %{"picture" => picture_params}) do
    file_size = case File.stat(picture_params["picture"].path) do
                  {:ok, record} -> record.size
                  _ -> 0
                end

    cleaned_params = if String.strip(to_string(picture_params["content"])) == "" do
      Map.put(picture_params, "content", picture_params["title"])
    else
      picture_params
    end |> Dict.delete("picture")

    changeset = Picture.changeset(%Picture{author_id: current_user(conn).id,
                                           image_file_name: picture_params["picture"].filename,
                                           image_content_type: picture_params["picture"].content_type,
                                           image_file_size: file_size,
                                           image_updated_at: Ecto.DateTime.utc()}, cleaned_params)

    case Repo.insert(changeset) do
      {:ok, picture} ->
        try do
          Picture.save_file(picture, picture_params["picture"].path)

          urls = case Webmentions.send_webmentions(picture_url(conn, :show, picture)) do
                   {:ok, list} ->
                     list
                   _ ->
                     []
                 end

          notice = "Picture created successfully. Webmentions sent to these endpoints:\n" <> Webmentions.results_as_text(urls)

          conn
          |> put_flash(:info, notice)
          |> redirect(to: picture_path(conn, :index))

        rescue
          e ->
            Logger.warn inspect(e)
            Picture.remove_file(picture)
            Repo.delete!(picture)

            conn |>
              put_flash(:error, "Error creating image") |>
              render("new.html", changeset: changeset, error: e)
        end
      {:error, changeset} ->
        render(conn, "new.html", changeset: changeset)
    end
  end

  def show(conn, params) do
    {id, suffix} = case params["id"] |> String.split(".") |> Enum.reverse do
                     [_] -> {params["id"], nil}
                     [format, id] -> {id, format}
                   end

    type = case params["type"] do
             "thumbnail" -> :thumbnail
             "large" -> :large
             _ -> :original
           end

    picture = Picture |> Picture.with_author |> Picture.with_mentions |> Repo.get!(id)

    if suffix == nil do
      exif_data = case ElixirExif.parse_file(Picture.file(picture, :original)) do
                    {:ok, fields, _} ->
                      fields
                    _ ->
                      %{}
                  end

      render(conn, "show.html", picture: picture, type: type, exif: exif_data)
    else
      cache_time = Timex.now |> Timex.shift(days: 360)

      conn
      |> put_resp_header("content-type", picture.image_content_type)
      |> put_resp_header("expires", cache_time |> Timex.format!("{RFC1123}"))
      |> put_resp_header("cache-control", "public,max-age=31536000")
      |> put_resp_header("last-modified", Picture.inserted_at_timex(picture) |> Timex.format!("{RFC1123}"))
      |> send_file(200, Picture.file(picture, type))
    end
  end

  def edit(conn, %{"id" => id}) do
    picture = Repo.get!(Picture, id)
    changeset = Picture.changeset(picture)
    render(conn, "edit.html", picture: picture, changeset: changeset)
  end

  def update(conn, %{"id" => id, "picture" => picture_params}) do
    picture = Repo.get!(Picture, id)
    changeset = Picture.changeset(picture, picture_params)

    case Repo.update(changeset) do
      {:ok, picture} ->
        if picture_params["picture"] do
          try do
            Picture.save_file(picture, picture_params["picture"].path)
          rescue
            e ->
              Logger.warn inspect(e)
              false
          end
        end

        urls = case Webmentions.send_webmentions(picture_url(conn, :show, picture)) do
                 {:ok, list} ->
                   list
                 _ ->
                   []
               end

        notice = "Picture updated successfully. Webmentions sent to these endpoints:\n" <> Webmentions.results_as_text(urls)


        conn
        |> put_flash(:info, notice)
        |> redirect(to: picture_path(conn, :show, picture))
      {:error, changeset} ->
        render(conn, "edit.html", picture: picture, changeset: changeset)
    end
  end

  def delete(conn, %{"id" => id}) do
    picture = Repo.get!(Picture, id)

    # Here we use delete! (with a bang) because we expect
    # it to always work (and if it does not, it will raise).
    Repo.delete!(picture)

    Picture.remove_file(picture)


    conn
    |> put_flash(:info, "Picture deleted successfully.")
    |> redirect(to: picture_path(conn, :index))
  end
end
