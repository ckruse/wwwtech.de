defmodule WwwtechWeb.NoteController do
  use WwwtechWeb, :controller

  alias Wwwtech.Notes
  alias Wwwtech.Notes.Note
  alias WwwtechWeb.Paging
  alias Wwwtech.Mentions

  plug :set_mention_header when action in [:index, :show]
  plug :set_caching_headers when action in [:index, :show]
  plug :require_login when action in [:new, :edit, :create, :update, :delete]

  def index(conn, params) do
    number_of_notes = Notes.count_notes(show_hidden: logged_in?(conn))
    paging = Paging.paginate(number_of_notes, page: params["p"])

    notes =
      Notes.list_notes(show_hidden: logged_in?(conn), with: [:author], limit: paging.limit, offset: paging.offset)
      |> Enum.reduce(%{}, fn note, nbd ->
        date = NaiveDateTime.to_date(note.inserted_at)
        Map.update(nbd, date, [note], &(&1 ++ [note]))
      end)

    keys =
      notes
      |> Map.keys()
      |> Enum.sort_by(fn d -> {d.year, d.month, d.day} end, &>=/2)

    render(conn, "index.html", notes: notes, dates: keys, paging: paging)
  end

  def index_atom(conn, _params) do
    notes = Notes.list_notes(limit: 50, offset: 0)

    callbacks = %{
      title: "WWWTech / Notes",
      id: Routes.note_url(conn, :index) <> ".atom",
      self_url: Routes.note_url(conn, :index) <> ".atom",
      alternate_url: Routes.note_url(conn, :index),
      entry_url: &Routes.note_url(conn, :show, &1),
      entry_id: &"tag:wwwtech.de,2005:Note/#{&1.id}",
      entry_title: & &1.title,
      entry_content: &Phoenix.View.render_to_string(WwwtechWeb.NoteView, "note.html", note: &1, atom: true, conn: conn)
    }

    conn
    |> put_resp_content_type("application/atom+xml", "utf-8")
    |> send_resp(200, WwwtechWeb.Atom.to_atom(notes, callbacks))
  end

  def new(conn, _params) do
    changeset = Notes.change_note(%Note{})
    render(conn, "new.html", changeset: changeset)
  end

  def create(conn, %{"note" => note_params}) do
    note_params =
      note_params
      |> Map.put("author_id", conn.assigns[:current_user].id)
      |> put_if_blank("content", note_params["title"])
      |> put_if_blank("title", note_params["content"])

    case Notes.create_note(note_params) do
      {:ok, note} ->
        info = Mentions.send_webmentions(Routes.note_url(conn, :show, note), "Note", "created")

        conn
        |> put_flash(:info, info)
        |> redirect(to: Routes.note_path(conn, :show, note))

      {:error, %Ecto.Changeset{} = changeset} ->
        render(conn, "new.html", changeset: changeset)
    end
  end

  def show(conn, %{"id" => id}) do
    note = Notes.get_note!(id, with: [:author])
    render(conn, "show.html", note: note)
  end

  def edit(conn, %{"id" => id}) do
    note = Notes.get_note!(id)
    changeset = Notes.change_note(note)
    render(conn, "edit.html", note: note, changeset: changeset)
  end

  def update(conn, %{"id" => id, "note" => note_params}) do
    note = Notes.get_note!(id)
    note_params = Map.drop(note_params, ["user_id"])

    case Notes.update_note(note, note_params) do
      {:ok, note} ->
        info = Mentions.send_webmentions(Routes.note_url(conn, :show, note), "Note", "updated")

        conn
        |> put_flash(:info, info)
        |> redirect(to: Routes.note_path(conn, :show, note))

      {:error, %Ecto.Changeset{} = changeset} ->
        render(conn, "edit.html", note: note, changeset: changeset)
    end
  end

  def delete(conn, %{"id" => id}) do
    note = Notes.get_note!(id)
    {:ok, _note} = Notes.delete_note(note)

    conn
    |> put_flash(:info, "Note deleted successfully.")
    |> redirect(to: Routes.note_path(conn, :index))
  end
end
