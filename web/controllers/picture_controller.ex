defmodule Wwwtech.PictureController do
  use Wwwtech.Web, :controller

  alias Wwwtech.Picture

  plug :require_login when action in [:new, :edit, :create, :update, :delete]
  plug :scrub_params, "picture" when action in [:create, :update]

  def index(conn, params) do
    page = Picture
    |> Picture.sorted
    |> Picture.with_author
    |> Repo.paginate(params)

    render(conn, "index.html",
           pictures: page.entries,
           page: page)
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

    changeset = Picture.changeset(%Picture{author_id: current_user(conn).id,
                                           image_file_name: picture_params["picture"].filename,
                                           image_content_type: picture_params["picture"].content_type,
                                           image_file_size: file_size,
                                           image_updated_at: Ecto.DateTime.utc()}, Dict.delete(picture_params, "picture"))

    case Repo.insert(changeset) do
      {:ok, picture} ->
        try do
          Picture.save_file(picture, picture_params["picture"].path)
          conn
          |> put_flash(:info, "Picture created successfully.")
          |> redirect(to: picture_path(conn, :index))

        rescue
          e ->
            IO.inspect(e)
            Repo.delete!(picture)
            render(conn, "new.html", changeset: changeset, error: e)
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

    picture = Picture |> Picture.with_author |> Repo.get!(id)

    if suffix == nil do
      render(conn, "show.html", picture: picture, type: type)
    else
      conn
      |> put_resp_header("Content-Type", picture.image_content_type)
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
        conn
        |> put_flash(:info, "Picture updated successfully.")
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
