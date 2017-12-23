defmodule Wwwtech.Notes do
  @moduledoc """
  The Indie context.
  """

  import Ecto.Query, warn: false
  alias Wwwtech.Repo

  alias Wwwtech.Notes.Note

  @doc """
  Returns the list of notes.

  ## Examples

      iex> list_notes()
      [%Note{}, ...]

  """
  def list_notes(only_visible \\ true, opts \\ [limit: nil]) do
    from(
      note in Note,
      preload: [:author, :mentions],
      order_by: [desc: note.inserted_at]
    )
    |> filter_visible(only_visible)
    |> Wwwtech.PagingApi.set_limit(opts[:limit])
    |> Repo.all()
  end

  @doc """
  Returns the number of notes.

  ## Examples

      iex> count_notes(true)
      1

      iex> count_notes(false)
      2
  """
  def count_notes(only_visible \\ true) do
    Note
    |> filter_visible(only_visible)
    |> Repo.aggregate(:count, :id)
  end

  defp filter_visible(query, true), do: where(query, show_in_index: true)
  defp filter_visible(query, _), do: query

  @doc """
  Gets a single note.

  Raises `Ecto.NoResultsError` if the Note does not exist.

  ## Examples

      iex> get_note!(123)
      %Note{}

      iex> get_note!(456)
      ** (Ecto.NoResultsError)

  """
  def get_note!(id) do
    from(
      note in Note,
      preload: [:author, :mentions],
      where: note.id == ^id
    )
    |> Repo.one!()
  end

  @doc """
  Creates a note.

  ## Examples

      iex> create_note(%{field: value})
      {:ok, %Note{}}

      iex> create_note(%{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def create_note(user, attrs \\ %{}) do
    attrs =
      if attrs["content"] == nil || String.trim(attrs["content"]) == "" do
        Map.put(attrs, "content", attrs["title"])
      else
        attrs
      end

    %Note{author_id: user.id}
    |> Note.changeset(attrs)
    |> Repo.insert()
  end

  @doc """
  Updates a note.

  ## Examples

      iex> update_note(note, %{field: new_value})
      {:ok, %Note{}}

      iex> update_note(note, %{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def update_note(%Note{} = note, attrs) do
    note
    |> Note.changeset(attrs)
    |> Repo.update()
  end

  @doc """
  Deletes a Note.

  ## Examples

      iex> delete_note(note)
      {:ok, %Note{}}

      iex> delete_note(note)
      {:error, %Ecto.Changeset{}}

  """
  def delete_note(%Note{} = note) do
    Repo.delete(note)
  end

  @doc """
  Returns an `%Ecto.Changeset{}` for tracking note changes.

  ## Examples

      iex> change_note(note)
      %Ecto.Changeset{source: %Note{}}

  """
  def change_note(%Note{} = note) do
    Note.changeset(note, %{})
  end
end
