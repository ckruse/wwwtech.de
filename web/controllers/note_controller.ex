defmodule Wwwtech.NoteController do
  use Wwwtech.Web, :controller
  use Wwwtech.Web, :web_controller

  alias Wwwtech.Note

  plug :set_mention_header when action in [:index, :show]
  plug :require_login when action in [:new, :edit, :create, :update, :delete]
  plug :scrub_params, "note" when action in [:create, :update]
  plug :set_caching_headers, only: [:index, :show]

  def index(conn, params) do
    page = Note
    |> Note.only_index(logged_in?(conn))
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

  def index_atom(conn, _params) do
    notes = Note
    |> Note.sorted
    |> Note.with_author
    |> Note.last_x(50)
    |> Repo.all

    render(conn, "index.atom", notes: notes)
  end

  def new(conn, _params) do
    changeset = Note.changeset(%Note{})
    render(conn, "new.html", changeset: changeset)
  end

  def create(conn, %{"note" => note_params}) do
    cleaned_params = if String.strip(to_string(note_params["content"])) == "" do
      Map.put(note_params, "content", note_params["title"])
    else
      note_params
    end

    changeset = Note.changeset(%Note{author_id: current_user(conn).id}, cleaned_params)

    case Repo.insert(changeset) do
      {:ok, note} ->
        urls = case Webmentions.send_webmentions(note_url(conn, :show, note)) do
                 {:ok, list} ->
                   list
                 _ ->
                   ["none"]
               end

        notice = "Note created successfully. Webmentions sent to these endpoints: " <> Enum.join(urls, ", ")

        conn
        |> put_flash(:info, notice)
        |> redirect(to: note_path(conn, :index))
      {:error, changeset} ->
        render(conn, "new.html", changeset: changeset)
    end
  end

  def show(conn, %{"id" => id}) do
    note = Note |> Note.with_author |> Note.with_mentions |> Repo.get!(id)
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
        urls = case Webmentions.send_webmentions(note_url(conn, :show, note)) do
                 {:ok, list} ->
                   list
                 _ ->
                   ["none"]
               end

        notice = "Note updated successfully. Webmentions sent to these endpoints: " <> Enum.join(urls, ", ")

        conn
        |> put_flash(:info, notice)
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
