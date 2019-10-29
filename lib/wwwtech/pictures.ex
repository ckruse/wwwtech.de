defmodule Wwwtech.Pictures do
  @moduledoc """
  The Pictures context.
  """

  import Ecto.Query, warn: false
  alias Wwwtech.Repo
  alias Wwwtech.EctoEnhancements

  alias Wwwtech.Pictures.Picture

  @doc """
  Returns the list of pictures.

  ## Examples

      iex> list_pictures()
      [%Picture{}, ...]

  """
  def list_pictures(opts \\ []) do
    opts = Keyword.merge([show_hidden: false, limit: 25, offset: 0], opts)

    Picture
    |> EctoEnhancements.filter_hidden(opts[:show_hidden])
    |> EctoEnhancements.apply_limit(opts[:limit], opts[:offset])
    |> order_by(desc: :inserted_at, desc: :id)
    |> Repo.all()
    |> Repo.maybe_preload(opts[:with])
  end

  def count_pictures(opts \\ []) do
    opts = Keyword.merge([show_hidden: false], opts)

    from(Picture, select: count())
    |> EctoEnhancements.filter_hidden(opts[:show_hidden])
    |> Repo.one()
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
  def get_picture!(id, opts \\ []) do
    Picture
    |> Repo.get!(id)
    |> Repo.maybe_preload(opts[:with])
  end

  @doc """
  Creates a picture.

  ## Examples

      iex> create_picture(%{field: value})
      {:ok, %Picture{}}

      iex> create_picture(%{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def create_picture(attrs \\ %{}, callback) do
    ret =
      %Picture{}
      |> Picture.changeset(attrs)
      |> Repo.insert()

    with {:ok, picture} <- ret do
      save_file(picture, attrs["picture"].path, callback)
      {:ok, picture}
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
    ret =
      picture
      |> Picture.changeset(attrs)
      |> Repo.update()

    with {:ok, picture} <- ret do
      if Wwwtech.Utils.present?(attrs["picture"]),
        do: save_file(picture, attrs["picture"].path)

      {:ok, picture}
    end
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
    with {:ok, img} <- Repo.delete(picture) do
      remove_file(picture)
      {:ok, img}
    end
  end

  def remove_file(picture) do
    path = storage_dir(picture)
    File.rm_rf(path)
  end

  def save_file(picture, upload_path, callback \\ nil) do
    path = storage_dir(picture)
    File.mkdir_p!(path <> "/original")
    File.mkdir_p!(path <> "/large")
    File.mkdir_p!(path <> "/thumbnail")

    File.cp!(upload_path, path <> "/original/#{picture.image_file_name}")

    generate_versions(picture, callback)
  end

  def generate_versions(picture, callback \\ nil) do
    spawn(fn ->
      generate_versions_synchronously(picture)
      if callback, do: callback.(picture)
    end)
  end

  def generate_versions_synchronously(picture) do
    path = storage_dir(picture)
    orig_path = "#{path}/original/#{picture.image_file_name}"

    orig_path
    |> Mogrify.open()
    |> Mogrify.copy()
    |> Mogrify.auto_orient()
    |> Mogrify.custom("strip")
    |> Mogrify.save()
    |> Mogrify.resize_to_fill("600x600")
    |> Mogrify.save(path: path <> "/thumbnail/#{picture.image_file_name}")

    orig_path
    |> Mogrify.open()
    |> Mogrify.copy()
    |> Mogrify.auto_orient()
    |> Mogrify.custom("strip")
    |> Mogrify.save()
    |> Mogrify.resize_to_limit("800x600>")
    |> Mogrify.save(path: path <> "/large/#{picture.image_file_name}")
  end

  def regen_all_pictures() do
    Picture
    |> Repo.all()
    |> Enum.each(&generate_versions_synchronously/1)
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

  defp storage_dir(picture), do: Application.get_env(:wwwtech, :storage_path) <> "/#{picture.id}"
  def filename(picture, type), do: storage_dir(picture) <> "/#{type}/#{picture.image_file_name}"
end
