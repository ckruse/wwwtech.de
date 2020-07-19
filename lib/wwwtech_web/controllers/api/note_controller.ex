defmodule WwwtechWeb.Api.NoteController do
  use WwwtechWeb, :controller

  alias Wwwtech.Notes
  alias Wwwtech.Notes.Note
  alias Wwwtech.Mentions

  action_fallback WwwtechWeb.FallbackController

  def index(conn, params) do
    page =
      if present?(params["p"]) && String.match?(params["p"], ~r/^\d+$/),
        do: String.to_integer(params["p"], 10),
        else: 0

    notes = Notes.list_notes(show_hidden: true, limit: 50, offset: page * 50)

    render(conn, "index.json", notes: notes)
  end

  def create(conn, %{"note" => note_params}) do
    note_params =
      note_params
      |> Map.put("author_id", conn.assigns[:current_user].id)
      |> put_if_blank("content", note_params["title"])
      |> put_if_blank("title", note_params["content"])

    with {:ok, %Note{} = note} <- Notes.create_note(note_params) do
      Task.start(fn -> Mentions.send_webmentions(Routes.note_url(conn, :show, note), "Note", "created") end)

      conn
      |> put_status(:created)
      |> put_resp_header("location", Routes.api_note_path(conn, :show, note))
      |> render("show.json", note: note)
    end
  end

  def show(conn, %{"id" => id}) do
    note = Notes.get_note!(id)
    render(conn, "show.json", note: note)
  end

  def update(conn, %{"id" => id, "note" => note_params}) do
    note = Notes.get_note!(id)
    note_params = Map.drop(note_params, ["user_id"])

    with {:ok, %Note{} = note} <- Notes.update_note(note, note_params) do
      Task.start(fn -> Mentions.send_webmentions(Routes.note_url(conn, :show, note), "Note", "updated") end)
      render(conn, "show.json", note: note)
    end
  end

  def delete(conn, %{"id" => id}) do
    note = Notes.get_note!(id)

    with {:ok, %Note{}} <- Notes.delete_note(note) do
      send_resp(conn, :no_content, "")
    end
  end
end
