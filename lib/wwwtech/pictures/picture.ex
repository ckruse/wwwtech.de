defmodule Wwwtech.Pictures.Picture do
  use Ecto.Schema
  import Ecto.Changeset

  schema "pictures" do
    field(:title, :string, null: false)
    field(:lang, :string, null: false, default: "en")
    field(:content, :string, null: false)
    field(:posse, :boolean, default: false)
    belongs_to(:author, Wwwtech.Accounts.Author)
    field(:in_reply_to, :string)
    field(:image_file_name, :string, null: false)
    field(:image_content_type, :string, null: false)
    field(:image_file_size, :integer, null: false)
    field(:image_updated_at, :naive_datetime)
    field(:show_in_index, :boolean, default: true, null: false)

    has_many(:mentions, Wwwtech.Mentions.Mention)

    timestamps()
  end

  @doc """
  Creates a changeset based on the `model` and `params`.

  If no params are provided, an invalid changeset is returned
  with no validation performed.
  """
  def changeset(model, params \\ %{}) do
    model
    |> cast(params, [
      :title,
      :lang,
      :content,
      :posse,
      :author_id,
      :image_file_name,
      :image_content_type,
      :image_file_size,
      :image_updated_at,
      :show_in_index,
      :in_reply_to
    ])
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

  def to_html(model) do
    Cmark.to_html(model.content)
  end
end
