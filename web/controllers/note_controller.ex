defmodule Wwwtech.NoteController do
  use Wwwtech.Web, :controller

  alias Wwwtech.Note

  plug :require_login when action in [:new, :edit, :create, :update, :delete]
  plug :scrub_params, "note" when action in [:create, :update]

  def index(conn, params) do
    page = Note
    |> Note.sorted
    |> Note.with_author
    |> Repo.paginate(params)

    {notes_by_day, keys} = Enum.reduce page.entries, {%{}, []}, fn note, {nbd, keys} ->
      {date, _} = Ecto.DateTime.to_erl(note.inserted_at)
      if nbd[date] == nil do
        {Map.put(nbd, date, [note]), keys ++ [date]}
      else
        {Map.put(nbd, date, nbd[date] ++ [note]), keys}
      end
    end

    render(conn, "index.html",
           page: page,
           notes: page.entries,
           notes_by_day: notes_by_day,
           days: keys)
  end

  def new(conn, _params) do
    changeset = Note.changeset(%Note{})
    render(conn, "new.html", changeset: changeset)
  end

  def create(conn, %{"note" => note_params}) do
    changeset = Note.changeset(%Note{}, note_params)

    case Repo.insert(changeset) do
      {:ok, _note} ->
        conn
        |> put_flash(:info, "Note created successfully.")
        |> redirect(to: note_path(conn, :index))
      {:error, changeset} ->
        render(conn, "new.html", changeset: changeset)
    end
  end

  def show(conn, %{"id" => id}) do
    note = Repo.get!(Note, id)
    render(conn, "show.html", note: note)
  end

  def edit(conn, %{"id" => id}) do
    note = Repo.get!(Note, id)
    changeset = Note.changeset(note)
    render(conn, "edit.html", note: note, changeset: changeset)
  end

  def update(conn, %{"id" => id, "note" => note_params}) do
    note = Repo.get!(Note, id)
    changeset = Note.changeset(note, note_params)

    case Repo.update(changeset) do
      {:ok, note} ->
        conn
        |> put_flash(:info, "Note updated successfully.")
        |> redirect(to: note_path(conn, :show, note))
      {:error, changeset} ->
        render(conn, "edit.html", note: note, changeset: changeset)
    end
  end

  def delete(conn, %{"id" => id}) do
    note = Repo.get!(Note, id)

    # Here we use delete! (with a bang) because we expect
    # it to always work (and if it does not, it will raise).
    Repo.delete!(note)

    conn
    |> put_flash(:info, "Note deleted successfully.")
    |> redirect(to: note_path(conn, :index))
  end
end
