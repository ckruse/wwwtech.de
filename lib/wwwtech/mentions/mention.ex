defmodule Wwwtech.Mentions.Mention do
  use Ecto.Schema
  import Ecto.Changeset

  schema "mentions" do
    field :source_url, :string
    field :target_url, :string
    field :title, :string
    field :author, :string
    field :author_avatar, :string
    field :author_url, :string
    field :mention_type, :string
    field :excerpt, :string

    belongs_to(:note, Wwwtech.Notes.Note)
    belongs_to(:picture, Wwwtech.Pictures.Picture)
    belongs_to(:article, Wwwtech.Articles.Article)

    timestamps()
  end

  @doc false
  def changeset(mention, attrs) do
    mention
    |> cast(attrs, [
      :source_url,
      :target_url,
      :author,
      :mention_type,
      :title,
      :excerpt,
      :author_url,
      :author_avatar,
      :note_id,
      :picture_id,
      :article_id
    ])
    |> validate_required([:source_url, :target_url, :author, :mention_type])
  end
end
