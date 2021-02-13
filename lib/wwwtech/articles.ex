defmodule Wwwtech.Articles do
  @moduledoc """
  The Articles context.
  """

  import Ecto.Query, warn: false
  alias Wwwtech.Repo
  alias Wwwtech.EctoEnhancements

  alias Wwwtech.Articles.Article

  @doc """
  Returns the list of articles.

  ## Examples

      iex> list_articles()
      [%Article{}, ...]

  """
  def list_articles(opts \\ []) do
    opts = Keyword.merge([show_hidden: false, limit: 25, offset: 0], opts)

    Article
    |> EctoEnhancements.filter_hidden(opts[:show_hidden], :published)
    |> EctoEnhancements.apply_limit(opts[:limit], opts[:offset])
    |> order_by(desc: :inserted_at, desc: :id)
    |> Repo.all()
    |> Repo.maybe_preload(opts[:with])
  end

  def count_articles(opts \\ []) do
    opts = Keyword.merge([show_hidden: false], opts)

    from(Article, select: count())
    |> EctoEnhancements.filter_hidden(opts[:show_hidden], :published)
    |> Repo.one()
  end

  @doc """
  Gets a single article.

  Raises `Ecto.NoResultsError` if the Article does not exist.

  ## Examples

      iex> get_article!(123)
      %Article{}

      iex> get_article!(456)
      ** (Ecto.NoResultsError)

  """
  def get_article!(id, opts \\ []) do
    Article
    |> Repo.get!(id)
    |> Repo.maybe_preload(opts[:with])
  end

  def get_article_by_slug!(slug, opts \\ []) do
    Article
    |> EctoEnhancements.filter_hidden(opts[:show_hidden], :published)
    |> Repo.get_by!(slug: slug)
    |> Repo.maybe_preload(opts[:with])
  end

  def get_last_article(opts \\ []) do
    from(article in Article, order_by: [desc: :inserted_at], limit: 1)
    |> EctoEnhancements.filter_hidden(opts[:show_hidden], :published)
    |> Repo.one()
    |> Repo.maybe_preload(opts[:with])
  end

  def search_article_by_slug_part(part, opts \\ []) do
    from(article in Article, where: like(article.slug, ^"%/#{part}"))
    |> EctoEnhancements.filter_hidden(opts[:show_hidden], :published)
    |> Repo.one!()
    |> Repo.maybe_preload(opts[:with])
  end

  @doc """
  Creates a article.

  ## Examples

      iex> create_article(%{field: value})
      {:ok, %Article{}}

      iex> create_article(%{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def create_article(attrs \\ %{}) do
    %Article{}
    |> Article.changeset(attrs)
    |> Repo.insert()
  end

  @doc """
  Updates a article.

  ## Examples

      iex> update_article(article, %{field: new_value})
      {:ok, %Article{}}

      iex> update_article(article, %{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def update_article(%Article{} = article, attrs) do
    article
    |> Article.changeset(attrs)
    |> Repo.update()
  end

  @doc """
  Deletes a Article.

  ## Examples

      iex> delete_article(article)
      {:ok, %Article{}}

      iex> delete_article(article)
      {:error, %Ecto.Changeset{}}

  """
  def delete_article(%Article{} = article) do
    Repo.delete(article)
  end

  @doc """
  Returns an `%Ecto.Changeset{}` for tracking article changes.

  ## Examples

      iex> change_article(article)
      %Ecto.Changeset{source: %Article{}}

  """
  def change_article(%Article{} = article) do
    Article.changeset(article, %{})
  end
end
