defmodule Wwwtech.Notes.Note do
  use Ecto.Schema
  import Ecto.Changeset

  schema "notes" do
    field :content, :string
    field :in_reply_to, :string
    field :lang, :string, default: "en"
    field :note_type, :string, default: "note"
    field :posse, :boolean, default: false
    field :show_in_index, :boolean, default: false
    field :title, :string

    has_many(:mentions, Wwwtech.Mentions.Mention)
    belongs_to(:author, Wwwtech.Accounts.Author)

    timestamps()
  end

  @doc false
  def changeset(note, attrs) do
    note
    |> cast(attrs, [:author_id, :title, :lang, :content, :in_reply_to, :posse, :show_in_index, :note_type])
    |> validate_required([:author_id, :title, :lang, :content, :show_in_index, :note_type])
  end
end
