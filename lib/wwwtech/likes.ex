defmodule Wwwtech.Likes do
  @moduledoc """
  The Likes context.
  """

  import Ecto.Query, warn: false
  alias Wwwtech.Repo
  alias Wwwtech.EctoEnhancements

  alias Wwwtech.Likes.Like

  @doc """
  Returns the list of likes.

  ## Examples

      iex> list_likes()
      [%Like{}, ...]

  """
  def list_likes(opts \\ []) do
    opts = Keyword.merge([show_hidden: false, limit: 25, offset: 0], opts)

    Like
    |> EctoEnhancements.filter_hidden(opts[:show_hidden])
    |> EctoEnhancements.apply_limit(opts[:limit], opts[:offset])
    |> order_by(desc: :inserted_at, desc: :id)
    |> Repo.all()
    |> Repo.maybe_preload(opts[:with])
  end

  def count_likes(opts \\ []) do
    opts = Keyword.merge([show_hidden: false], opts)

    from(Like, select: count())
    |> EctoEnhancements.filter_hidden(opts[:show_hidden])
    |> Repo.one()
  end

  @doc """
  Gets a single like.

  Raises `Ecto.NoResultsError` if the Like does not exist.

  ## Examples

      iex> get_like!(123)
      %Like{}

      iex> get_like!(456)
      ** (Ecto.NoResultsError)

  """
  def get_like!(id, opts \\ []) do
    Like
    |> Repo.get!(id)
    |> Repo.maybe_preload(opts[:with])
  end

  @doc """
  Creates a like.

  ## Examples

      iex> create_like(%{field: value})
      {:ok, %Like{}}

      iex> create_like(%{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def create_like(attrs \\ %{}) do
    %Like{}
    |> Like.changeset(attrs)
    |> Repo.insert()
  end

  @doc """
  Updates a like.

  ## Examples

      iex> update_like(like, %{field: new_value})
      {:ok, %Like{}}

      iex> update_like(like, %{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def update_like(%Like{} = like, attrs) do
    like
    |> Like.changeset(attrs)
    |> Repo.update()
  end

  @doc """
  Deletes a Like.

  ## Examples

      iex> delete_like(like)
      {:ok, %Like{}}

      iex> delete_like(like)
      {:error, %Ecto.Changeset{}}

  """
  def delete_like(%Like{} = like) do
    Repo.delete(like)
  end

  @doc """
  Returns an `%Ecto.Changeset{}` for tracking like changes.

  ## Examples

      iex> change_like(like)
      %Ecto.Changeset{source: %Like{}}

  """
  def change_like(%Like{} = like) do
    Like.changeset(like, %{})
  end
end
