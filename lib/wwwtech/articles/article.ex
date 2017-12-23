defmodule Wwwtech.Articles.Article do
  use Ecto.Schema
  import Ecto.Changeset
  alias Wwwtech.Articles.Article

  use Timex
  use Timex.Ecto.Timestamps

  schema "articles" do
    field(:in_reply_to, :string)
    field(:title, :string)
    field(:slug, :string)
    field(:lang, :string, default: "en")
    field(:guid, :string)
    field(:article_format, :string)
    field(:excerpt, :string)
    field(:body, :string)
    field(:published, :boolean, default: false)
    field(:posse, :boolean, default: false)

    belongs_to(:author, Wwwtech.Accounts.Author)
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
    |> cast(params, [:guid, :slug, :title, :lang, :excerpt, :body, :published, :posse, :in_reply_to])
    |> validate_required([:guid, :slug, :title, :lang, :body, :published, :posse])
    |> unique_constraint(:slug)
    |> unique_constraint(:guid)
  end

  def to_html(article) do
    if article.article_format == "markdown" do
      Cmark.to_html(article.body)
    else
      article.body
    end
  end

  def excerpt_to_html(%Article{excerpt: nil}), do: ""

  def excerpt_to_html(article) do
    if article.article_format == "markdown" do
      Cmark.to_html(article.excerpt)
    else
      article.excerpt
    end
  end
end
