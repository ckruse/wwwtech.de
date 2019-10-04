defmodule Wwwtech.Mentions do
  @moduledoc """
  The Mentions context.
  """

  import Ecto.Query, warn: false
  alias Wwwtech.Repo
  alias Wwwtech.EctoEnhancements

  alias Wwwtech.Mentions.Mention

  @doc """
  Returns the list of mentions.

  ## Examples

      iex> list_mentions()
      [%Mention{}, ...]

  """
  def list_mentions(opts \\ []) do
    opts = Keyword.merge([limit: 25, offset: 0], opts)

    Mention
    |> EctoEnhancements.apply_limit(opts[:limit], opts[:offset])
    |> order_by(desc: :inserted_at)
    |> Repo.all()
    |> Repo.maybe_preload(opts[:with])
  end

  def count_mentions() do
    from(Mention, select: count())
    |> Repo.one()
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
  def get_mention!(id), do: Repo.get!(Mention, id)

  def get_mention_by_source_and_target(source, target) do
    from(mention in Mention, where: mention.source_url == ^source and mention.target_url == ^target)
    |> Repo.one()
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
