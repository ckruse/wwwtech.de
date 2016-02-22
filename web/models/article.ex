defmodule Wwwtech.Article do
  use Wwwtech.Web, :model

  schema "articles" do
    belongs_to :author, Wwwtech.Author

    field :in_reply_to, :string
    field :title, :string
    field :slug, :string
    field :guid, :string
    field :article_format, :string
    field :excerpt, :string
    field :body, :string
    field :published, :boolean, default: false
    field :posse, :boolean, default: false

    timestamps
  end

  @required_fields ~w(author_id title slug guid article_format excerpt body published posse)
  @optional_fields ~w(in_reply_to)

  @doc """
  Creates a changeset based on the `model` and `params`.

  If no params are provided, an invalid changeset is returned
  with no validation performed.
  """
  def changeset(model, params \\ :empty) do
    model
    |> cast(params, @required_fields, @optional_fields)
    |> unique_constraint(:slug)
    |> unique_constraint(:guid)
  end

  def with_author(query) do
    query
    |> preload([:author])
  end

  def sorted(query) do
    query
    |> order_by([n], desc: n.inserted_at)
  end

  def sorted_asc(query) do
    query
    |> order_by([n], asc: n.inserted_at)
  end

  def only_visible(query, visibility) do
    if visibility == true do
      query
    else
      query |> where(published: true)
    end
  end

  def by_slug(query, slug) do
    query
    |> where(slug: ^slug)
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

  def to_html(article) do
    if article.article_format == "markdown" do
      Cmark.to_html article.body
    else
      article.body
    end
  end

  def excerpt_to_html(article) do
    if article.article_format == "markdown" do
      Cmark.to_html article.excerpt
    else
      article.excerpt
    end
  end
end
