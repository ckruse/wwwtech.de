defmodule Wwwtech.Articles.Article do
  use Ecto.Schema
  import Ecto.Changeset

  schema "articles" do
    field :article_format, :string, default: "markdown"
    field :body, :string
    field :excerpt, :string
    field :guid, :string
    field :in_reply_to, :string
    field :lang, :string, default: "en"
    field :posse, :boolean, default: false
    field :published, :boolean, default: false
    field :slug, :string
    field :title, :string

    has_many(:mentions, Wwwtech.Mentions.Mention)
    belongs_to(:author, Wwwtech.Accounts.Author)

    timestamps()
  end

  @doc false
  def changeset(article, attrs) do
    article
    |> cast(attrs, [:author_id, :guid, :slug, :title, :lang, :excerpt, :body, :published, :posse, :in_reply_to])
    |> validate_required([:author_id, :guid, :slug, :title, :lang, :body, :published, :posse])
    |> unique_constraint(:slug)
    |> unique_constraint(:guid)
  end
end
