defmodule Wwwtech.Mentions do
  @moduledoc """
  The Indie context.
  """

  import Ecto.Query, warn: false
  alias Wwwtech.Repo

  alias Wwwtech.Mentions.Mention

  @doc """
  Returns the list of mentions.

  ## Examples

      iex> list_mentions()
      [%Mention{}, ...]

  """
  def list_mentions(opts \\ [limit: nil]) do
    from(mention in Mention,
      order_by: [desc: mention.inserted_at])
    |> Wwwtech.PagingApi.set_limit(opts[:limit])
    |> Repo.all
  end

  @doc """
  Returns the number of mentions.

  ## Examples

      iex> count_mentions(true)
      1

      iex> count_mentions(false)
      2
  """
  def count_mentions do
    Mention
    |> Repo.aggregate(:count, :id)
  end

  @doc """
  Gets a single mention.

  Raises `Ecto.NoResultsError` if the Mention does not exist.

  ## Examples

      iex> get_mention!(123)
      %Mention{}

      iex> get_mention!(456)
      ** (Ecto.NoResultsError)

  """
  def get_mention!(id) do
    from(mention in Mention,
      where: mention.id == ^id)
    |> Repo.one!
  end

  def get_mention_by_source_and_target(source, target) do
    from(mention in Mention,
      where: mention.source_url == ^source and mention.target_url == ^target)
    |> Repo.one
  end

  @doc """
  Creates a mention.

  ## Examples

      iex> create_mention(%{field: value})
      {:ok, %Mention{}}

      iex> create_mention(%{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def create_mention(attrs \\ %{}) do
    %Mention{}
    |> Mention.changeset(attrs)
    |> Repo.insert()
  end

  @doc """
  Updates a mention.

  ## Examples

      iex> update_mention(mention, %{field: new_value})
      {:ok, %Mention{}}

      iex> update_mention(mention, %{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def update_mention(%Mention{} = mention, attrs) do
    mention
    |> Mention.changeset(attrs)
    |> Repo.update()
  end

  @doc """
  Deletes a Mention.

  ## Examples

      iex> delete_mention(mention)
      {:ok, %Mention{}}

      iex> delete_mention(mention)
      {:error, %Ecto.Changeset{}}

  """
  def delete_mention(%Mention{} = mention) do
    Repo.delete(mention)
  end

  @doc """
  Returns an `%Ecto.Changeset{}` for tracking mention changes.

  ## Examples

      iex> change_mention(mention)
      %Ecto.Changeset{source: %Mention{}}

  """
  def change_mention(%Mention{} = mention) do
    Mention.changeset(mention, %{})
  end
end
