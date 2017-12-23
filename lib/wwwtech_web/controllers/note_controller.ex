defmodule WwwtechWeb.NoteController do
  use WwwtechWeb.Web, :controller
  use WwwtechWeb.Web, :web_controller

  alias WwwtechWeb.Helpers.Paging

  alias Wwwtech.Notes
  alias Wwwtech.Notes.Note

  plug(:set_mention_header when action in [:index, :show])
  plug(:require_login when action in [:new, :edit, :create, :update, :delete])
  plug(:scrub_params, "note" when action in [:create, :update])
  plug(:set_caching_headers, only: [:index, :show])

  def index(conn, params) do
    number_of_notes = Notes.count_notes(!logged_in?(conn))
    paging = Paging.paginate(number_of_notes, page: params["p"])
    notes = Notes.list_notes(!logged_in?(conn), limit: paging.params)

    {notes_by_day, keys} =
      Enum.reduce(notes, {%{}, []}, fn note, {nbd, keys} ->
        {date, _} = Timex.to_erl(note.inserted_at)

        if nbd[date] == nil do
          {Map.put(nbd, date, [note]), keys ++ [date]}
        else
          {Map.put(nbd, date, nbd[date] ++ [note]), keys}
        end
      end)

    render(
      conn,
      "index.html",
      paging: paging,
      notes: notes,
      notes_by_day: notes_by_day,
      days: keys
    )
  end

  def index_atom(conn, _params) do
    notes = Notes.list_notes(true, limit: [quantity: 50, offset: 0])
    render(conn, "index.atom", notes: notes)
  end

  def new(conn, _params) do
    changeset = Notes.change_note(%Note{})
    render(conn, "new.html", changeset: changeset)
  end

  def create(conn, %{"note" => note_params}) do
    case Notes.create_note(current_user(conn), note_params) do
      {:ok, note} ->
        conn
        |> put_flash(
          :info,
          WwwtechWeb.Helpers.Webmentions.send_webmentions(note_url(conn, :show, note), "Note", "created")
        )
        |> redirect(to: note_path(conn, :index))

      {:error, %Ecto.Changeset{} = changeset} ->
        render(conn, "new.html", changeset: changeset)
    end
  end

  def show(conn, %{"id" => id}) do
    note = Notes.get_note!(id)
    render(conn, "show.html", note: note)
  end

  def edit(conn, %{"id" => id}) do
    note = Notes.get_note!(id)
    changeset = Notes.change_note(note)
    render(conn, "edit.html", note: note, changeset: changeset)
  end

  def update(conn, %{"id" => id, "note" => note_params}) do
    note = Notes.get_note!(id)

    case Notes.update_note(note, note_params) do
      {:ok, note} ->
        conn
        |> put_flash(
          :info,
          WwwtechWeb.Helpers.Webmentions.send_webmentions(note_url(conn, :show, note), "Note", "created")
        )
        |> redirect(to: note_path(conn, :index))

      {:error, %Ecto.Changeset{} = changeset} ->
        render(conn, "edit.html", note: note, changeset: changeset)
    end
  end

  def delete(conn, %{"id" => id}) do
    note = Notes.get_note!(id)

    # Here we use delete! (with a bang) because we expect
    # it to always work (and if it does not, it will raise).
    Notes.delete_note(note)

    conn
    |> put_flash(:info, "Note deleted successfully.")
    |> redirect(to: note_path(conn, :index))
  end
end
