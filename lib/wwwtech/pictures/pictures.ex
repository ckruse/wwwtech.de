defmodule Wwwtech.Pictures do
  @moduledoc """
  The Accounts context.
  """

  import Ecto.Query, warn: false
  alias Wwwtech.Repo

  alias Wwwtech.Pictures.Picture

  defp filter_visible(query, true), do: where(query, show_in_index: true)
  defp filter_visible(query, _), do: query

  @doc """
  Returns the list of pictures.

  ## Examples

      iex> list_pictures()
      [%Picture{}, ...]

  """
  def list_pictures(only_visible \\ true, opts \\ [limit: nil]) do
    from(
      picture in Picture,
      preload: [:author, :mentions],
      order_by: [desc: picture.inserted_at]
    )
    |> filter_visible(only_visible)
    |> Wwwtech.PagingApi.set_limit(opts[:limit])
    |> Repo.all()
  end

  @doc """
  Returns the number of pictures.

  ## Examples

      iex> count_pictures(true)
      1

      iex> count_pictures(false)
      2
  """
  def count_pictures(only_visible \\ true) do
    Picture
    |> filter_visible(only_visible)
    |> Repo.aggregate(:count, :id)
  end

  @doc """
  Gets a single picture.

  Raises `Ecto.NoResultsError` if the Picture does not exist.

  ## Examples

      iex> get_picture!(123)
      %Picture{}

      iex> get_picture!(456)
      ** (Ecto.NoResultsError)

  """
  def get_picture!(id) do
    from(
      picture in Picture,
      preload: [:author, :mentions],
      where: picture.id == ^id
    )
    |> Repo.one!()
  end

  defp storage_dir(picture), do: Application.get_env(:wwwtech, :storage_path) <> "/#{picture.id}"
  def filename(picture, type), do: storage_dir(picture) <> "/#{type}/#{picture.image_file_name}"

  @doc """
  Creates a picture.

  ## Examples

      iex> create_picture(%{field: value})
      {:ok, %Picture{}}

      iex> create_picture(%{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def create_picture(user, attrs \\ %{}) do
    pic = attrs["picture"]

    file_size =
      case File.stat(pic.path) do
        {:ok, record} -> record.size
        _ -> 0
      end

    attrs =
      if String.trim(to_string(attrs["content"])) == "" do
        Map.put(attrs, "content", attrs["title"])
      else
        attrs
      end
      |> Map.delete("picture")

    ret =
      %Picture{
        author_id: user.id,
        image_file_name: pic.filename,
        image_content_type: pic.content_type,
        image_file_size: file_size,
        image_updated_at: Timex.now() |> Timex.to_naive_datetime() |> NaiveDateTime.truncate(:second)
      }
      |> Picture.changeset(attrs)
      |> Repo.insert()

    case ret do
      {:ok, picture} ->
        save_file(picture, pic.path)
        ret

      _ ->
        ret
    end
  end

  @doc """
  Updates a picture.

  ## Examples

      iex> update_picture(picture, %{field: new_value})
      {:ok, %Picture{}}

      iex> update_picture(picture, %{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def update_picture(%Picture{} = picture, attrs) do
    pic = attrs["picture"]

    if pic, do: save_file(picture, pic.path)

    picture
    |> Picture.changeset(attrs)
    |> Repo.update()
  end

  @doc """
  Deletes a Picture.

  ## Examples

      iex> delete_picture(picture)
      {:ok, %Picture{}}

      iex> delete_picture(picture)
      {:error, %Ecto.Changeset{}}

  """
  def delete_picture(%Picture{} = picture) do
    Repo.delete(picture)
    remove_file(picture)
  end

  def remove_file(picture) do
    path = storage_dir(picture)
    File.rm_rf!(path)
  end

  def save_file(picture, upload_path) do
    path = storage_dir(picture)
    File.mkdir_p!(path <> "/original")
    File.mkdir_p!(path <> "/large")
    File.mkdir_p!(path <> "/thumbnail")

    File.cp!(upload_path, path <> "/original/#{picture.image_file_name}")

    spawn(fn ->
      Mogrify.open(upload_path)
      |> Mogrify.copy()
      |> Mogrify.auto_orient()
      |> Mogrify.custom("strip")
      |> Mogrify.save()
      |> Mogrify.resize_to_fill("150x150")
      |> Mogrify.save(path: path <> "/thumbnail/#{picture.image_file_name}")

      Mogrify.open(upload_path)
      |> Mogrify.copy()
      |> Mogrify.auto_orient()
      |> Mogrify.custom("strip")
      |> Mogrify.save()
      |> Mogrify.resize_to_limit("800x600>")
      |> Mogrify.save(path: path <> "/large/#{picture.image_file_name}")
    end)
  end

  @doc """
  Returns an `%Ecto.Changeset{}` for tracking picture changes.

  ## Examples

      iex> change_picture(picture)
      %Ecto.Changeset{source: %Picture{}}

  """
  def change_picture(%Picture{} = picture) do
    Picture.changeset(picture, %{})
  end
end
