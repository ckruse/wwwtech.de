defmodule Wwwtech.Notes do
  @moduledoc """
  The Notes context.
  """

  import Ecto.Query, warn: false
  alias Wwwtech.Repo
  alias Wwwtech.EctoEnhancements

  alias Wwwtech.Notes.Note

  @doc """
  Returns the list of notes.

  ## Examples

      iex> list_notes()
      [%Note{}, ...]

  """
  def list_notes(opts \\ []) do
    opts = Keyword.merge([show_hidden: false, limit: 50, offset: 0], opts)

    Note
    |> EctoEnhancements.filter_hidden(opts[:show_hidden])
    |> EctoEnhancements.apply_limit(opts[:limit], opts[:offset])
    |> order_by(desc: :inserted_at, desc: :id)
    |> Repo.all()
    |> Repo.maybe_preload(opts[:with])
  end

  def count_notes(opts \\ []) do
    opts = Keyword.merge([show_hidden: false], opts)

    from(Note, select: count())
    |> EctoEnhancements.filter_hidden(opts[:show_hidden])
    |> Repo.one()
  end

  @doc """
  Gets a single note.

  Raises `Ecto.NoResultsError` if the Note does not exist.

  ## Examples

      iex> get_note!(123)
      %Note{}

      iex> get_note!(456)
      ** (Ecto.NoResultsError)

  """
  def get_note!(id, opts \\ []) do
    Note
    |> Repo.get!(id)
    |> Repo.maybe_preload(opts[:with])
  end

  @doc """
  Creates a note.

  ## Examples

      iex> create_note(%{field: value})
      {:ok, %Note{}}

      iex> create_note(%{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def create_note(attrs \\ %{}) do
    %Note{}
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
