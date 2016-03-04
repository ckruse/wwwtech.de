defmodule Wwwtech.Mention do
  use Wwwtech.Web, :model

  schema "mentions" do
    belongs_to :note, Wwwtech.Note
    belongs_to :picture, Wwwtech.Picture
    belongs_to :article, Wwwtech.Article

    field :source_url, :string, null: false
    field :target_url, :string, null: false
    field :title, :string
    field :excerpt, :string
    field :author, :string, null: false
    field :author_url, :string
    field :author_avatar, :string
    field :mention_type, :string, null: false

    timestamps
  end

  @required_fields ~w(source_url target_url author mention_type)
  @optional_fields ~w(title excerpt author_url author_avatar note_id picture_id article_id)

  @doc """
  Creates a changeset based on the `model` and `params`.

  If no params are provided, an invalid changeset is returned
  with no validation performed.
  """
  def changeset(model, params \\ :empty) do
    model
    |> cast(params, @required_fields, @optional_fields)
  end

  def by_source_and_target(query, source, target) do
    query |> where(source_url: ^source, target_url: ^target)
  end

  def inserted_at_timex(mention) do
    Ecto.DateTime.to_erl(mention.inserted_at)
    |> Timex.Date.from
  end

  def sorted(query) do
    query
    |> order_by([n], desc: n.inserted_at)
  end

end
