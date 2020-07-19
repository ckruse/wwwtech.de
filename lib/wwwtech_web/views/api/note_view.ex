defmodule WwwtechWeb.Api.NoteView do
  use WwwtechWeb, :view
  alias WwwtechWeb.Api.NoteView

  def render("index.json", %{notes: notes}) do
    render_many(notes, NoteView, "note.json")
  end

  def render("show.json", %{note: note}) do
    render_one(note, NoteView, "note.json")
  end

  def render("note.json", %{note: note}) do
    %{
      id: note.id,
      content: note.content,
      in_reply_to: note.in_reply_to,
      lang: note.lang,
      note_type: note.note_type,
      posse: note.posse,
      show_in_index: note.show_in_index,
      title: note.title,
      inserted_at: note.inserted_at |> Timex.local() |> Timex.format!("{ISO:Extended:Z}"),
      updated_at: note.updated_at |> Timex.local() |> Timex.format!("{ISO:Extended:Z}")
    }
  end
end
