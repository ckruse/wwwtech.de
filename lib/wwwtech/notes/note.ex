defmodule Wwwtech.Notes.Note do
  use Ecto.Schema
  import Ecto.Changeset
  alias Wwwtech.Notes.Note

  schema "notes" do
    field(:title, :string, null: false)
    field(:lang, :string, null: false, default: "en")
    field(:content, :string, null: false)
    field(:in_reply_to, :string)
    field(:posse, :boolean, default: false, null: false)
    field(:show_in_index, :boolean, default: true, null: false)
    field(:note_type, :string, default: "note", null: false)

    belongs_to(:author, Wwwtech.Accounts.Author)
    has_many(:mentions, Wwwtech.Mentions.Mention)

    timestamps()
  end

  @doc """
  Creates a changeset based on the `model` and `params`.

  If no params are provided, an invalid changeset is returned
  with no validation performed.
  """
  def changeset(%Note{} = note, params \\ %{}) do
    note
    |> cast(params, [:title, :lang, :content, :posse, :show_in_index, :note_type, :in_reply_to])
    |> validate_required([:title, :lang, :content, :posse, :show_in_index, :note_type])
  end

  def to_html(model) do
    Cmark.to_html(model.content)
  end

  def today?(note) do
    Timex.compare(Timex.to_date(note.inserted_at), Timex.today()) == 0
  end

  def yesterday?(note) do
    date = Timex.today() |> Timex.shift(days: -1)
    Timex.compare(date, Timex.to_date(note.inserted_at)) == 0
  end
end
