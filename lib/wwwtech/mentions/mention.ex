defmodule Wwwtech.Mentions.Mention do
  use Ecto.Schema
  import Ecto.Changeset

  use Timex
  use Timex.Ecto.Timestamps

  schema "mentions" do
    belongs_to(:note, Wwwtech.Notes.Note)
    belongs_to(:picture, Wwwtech.Pictures.Picture)
    belongs_to(:article, Wwwtech.Articles.Article)

    field(:source_url, :string, null: false)
    field(:target_url, :string, null: false)
    field(:title, :string)
    field(:excerpt, :string)
    field(:author, :string, null: false)
    field(:author_url, :string)
    field(:author_avatar, :string)
    field(:mention_type, :string, null: false)

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
