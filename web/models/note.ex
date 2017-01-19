defmodule Wwwtech.Note do
  use Wwwtech.Web, :model

  schema "notes" do
    field :title, :string, null: false
    field :lang, :string, null: false, default: "en"
    field :content, :string, null: false
    field :in_reply_to, :string
    field :posse, :boolean, default: false, null: false
    field :show_in_index, :boolean, default: true, null: false
    field :note_type, :string, default: "note", null: false

    belongs_to :author, Wwwtech.Author
    has_many :mentions, Wwwtech.Mention

    timestamps()
  end

  @required_fields ~w(author_id title lang content posse show_in_index note_type)
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

  def with_mentions(query) do
    query
    |> preload([:mentions])
  end


  def sorted(query) do
    query
    |> order_by([n], desc: n.inserted_at)
  end

  def last_x(query, x) do
    query
    |> limit(^x)
  end

  def to_html(model) do
    Cmark.to_html model.content
  end

  def today?(note) do
     Timex.compare(note.inserted_at, Timex.now, :days) == 0
  end

  def yesterday?(note) do
    date = Timex.now |> Timex.shift(days: -1)
    Timex.compare(date, note.inserted_at, :days) == 0
  end
end
