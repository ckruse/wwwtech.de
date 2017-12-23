defmodule Wwwtech.Likes do
  @moduledoc """
  The Indie context.
  """

  import Ecto.Query, warn: false
  alias Wwwtech.Repo

  alias Wwwtech.Likes.Like

  defp filter_visible(query, true), do: where(query, show_in_index: true)
  defp filter_visible(query, _), do: query

  @doc """
  Returns the list of likes.

  ## Examples

      iex> list_likes()
      [%Like{}, ...]

  """
  def list_likes(only_visible \\ true, opts \\ [limit: nil]) do
    from(
      like in Like,
      preload: [:author],
      order_by: [desc: like.inserted_at]
    )
    |> filter_visible(only_visible)
    |> Wwwtech.PagingApi.set_limit(opts[:limit])
    |> Repo.all()
  end

  @doc """
  Returns the number of likes.

  ## Examples

      iex> list_likes(true)
      1

      iex> list_likes(false)
      2
  """
  def count_likes(only_visible \\ true) do
    Like
    |> filter_visible(only_visible)
    |> Repo.aggregate(:count, :id)
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
  def get_like!(id) do
    from(
      like in Like,
      preload: [:author],
      where: like.id == ^id
    )
    |> Repo.one!()
  end

  @doc """
  Creates a like.

  ## Examples

      iex> create_like(%{field: value})
      {:ok, %Like{}}

      iex> create_like(%{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def create_like(user, attrs \\ %{}) do
    %Like{author_id: user.id}
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
