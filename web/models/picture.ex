defmodule Wwwtech.Picture do
  use Wwwtech.Web, :model

  schema "pictures" do
    field :title, :string, null: false
    field :posse, :boolean, default: false
    belongs_to :author, Wwwtech.Author
    field :in_reply_to, :string
    field :image_file_name, :string, null: false
    field :image_content_type, :string, null: false
    field :image_file_size, :integer, null: false
    field :image_updated_at, Ecto.DateTime
    field :show_in_index, :boolean, default: true, null: false

    timestamps([type: Ecto.DateTime, usec: false, inserted_at: :created_at, updated_at: :updated_at])
  end

  @required_fields ~w(title posse author_id image_file_name image_content_type image_file_size image_updated_at show_in_index)
  @optional_fields ~w(in_reply_to)

  @doc """
  Creates a changeset based on the `model` and `params`.

  If no params are provided, an invalid changeset is returned
  with no validation performed.
  """
  def changeset(model, params \\ :empty) do
    model
    |> cast(params, @required_fields, @optional_fields)
  end

  def only_index(query, logged_in) do
    if logged_in == true do
      query
    else
      query |> where(show_in_index: true)
    end
  end

  def with_author(query) do
    query
    |> preload([:author])
  end

  def sorted(query) do
    query
    |> order_by([n], desc: n.created_at)
  end

  def last_x(query, x) do
    query
    |> limit(^x)
  end

  def created_at_timex(note) do
    Ecto.DateTime.to_erl(note.created_at)
    |> Timex.Date.from
  end

  def updated_at_timex(note) do
    Ecto.DateTime.to_erl(note.created_at)
    |> Timex.Date.from
  end

  def dir(picture) do
    Wwwtech.Endpoint.config(:root) <> "/pictures/#{picture.id}"
  end

  def file(picture, type) do
    Wwwtech.Picture.dir(picture) <> "/#{type}/#{picture.image_file_name}"
  end

  def save_file(picture, upload_path) do
    path = Wwwtech.Picture.dir(picture)
    File.mkdir_p!(path <> "/original")
    File.mkdir_p!(path <> "/large")
    File.mkdir_p!(path <> "/thumbnail")

    File.cp!(upload_path, path <> "/original/#{picture.image_file_name}")
    Mogrify.open(upload_path) |>
      Mogrify.copy |>
      Mogrify.resize_to_fill("150x150") |>
      Mogrify.save(path <> "/thumbnail/#{picture.image_file_name}")

    Mogrify.open(upload_path) |>
      Mogrify.copy |>
      Mogrify.resize_to_limit("800x600>") |>
      Mogrify.save(path <> "/large/#{picture.image_file_name}")
  end

  def remove_file(picture) do
    path = Wwwtech.Picture.dir(picture)
    File.rm_rf!(path)
  end
end
