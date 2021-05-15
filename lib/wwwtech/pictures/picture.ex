defmodule Wwwtech.Pictures.Picture do
  use Ecto.Schema
  import Ecto.Changeset

  schema "pictures" do
    field :title, :string
    field :lang, :string, default: "en"
    field :alt, :string
    field :content, :string

    field :in_reply_to, :string
    field :posse, :boolean, default: false
    field :show_in_index, :boolean, default: false

    field :image_content_type, :string
    field :image_file_name, :string
    field :image_file_size, :integer
    field :image_updated_at, :naive_datetime

    has_many(:mentions, Wwwtech.Mentions.Mention)
    belongs_to(:author, Wwwtech.Accounts.Author)

    timestamps()
  end

  @doc false
  def changeset(picture, attrs) do
    picture
    |> cast(attrs, [
      :title,
      :lang,
      :alt,
      :content,
      :posse,
      :author_id,
      :in_reply_to,
      :show_in_index,
      :image_file_name,
      :image_content_type,
      :image_file_size,
      :image_updated_at
    ])
    |> maybe_put_image_params(attrs["picture"])
    |> validate_required([
      :title,
      :lang,
      :content,
      :posse,
      :author_id,
      :image_file_name,
      :image_content_type,
      :image_file_size,
      :image_updated_at,
      :show_in_index
    ])
  end

  def maybe_put_image_params(%Ecto.Changeset{valid?: true} = changeset, %Plug.Upload{} = file) do
    file_size =
      case File.stat(file.path) do
        {:ok, record} -> record.size
        _ -> 0
      end

    now =
      NaiveDateTime.utc_now()
      |> NaiveDateTime.truncate(:second)

    changeset
    |> put_change(:image_file_name, file.filename)
    |> put_change(:image_content_type, file.content_type)
    |> put_change(:image_file_size, file_size)
    |> put_change(:image_updated_at, now)
  end

  def maybe_put_image_params(%Ecto.Changeset{valid?: true} = changeset, {:data, data}) do
    now =
      NaiveDateTime.utc_now()
      |> NaiveDateTime.truncate(:second)

    changeset
    |> put_change(:image_file_name, "img.jpg")
    |> put_change(:image_content_type, "image/jpeg")
    |> put_change(:image_file_size, String.length(data))
    |> put_change(:image_updated_at, now)
  end

  def maybe_put_image_params(changeset, _), do: changeset
end
