defmodule Wwwtech.Articles do
  import Ecto.Query, warn: false
  alias Wwwtech.Repo

  alias Wwwtech.Articles.Article

  @doc """
  Returns the list of articles.

  ## Examples

      iex> list_articles()
      [%Article{}, ...]

  """
  def list_articles(only_visible \\ true, opts \\ [limit: nil]) do
    from(
      article in Article,
      preload: [:author, :mentions],
      order_by: [desc: article.inserted_at]
    )
    |> filter_visible_articles(only_visible)
    |> Wwwtech.PagingApi.set_limit(opts[:limit])
    |> Repo.all()
  end

  defp filter_visible_articles(query, true), do: where(query, published: true)
  defp filter_visible_articles(query, false), do: query

  @doc """
  Returns the number of articles.

  ## Examples

      iex> list_articles(true)
      1

      iex> list_articles(false)
      2
  """
  def count_articles(only_visible \\ true) do
    Article
    |> filter_visible_articles(only_visible)
    |> Repo.aggregate(:count, :id)
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
  def get_article!(id, only_visible \\ true) do
    from(
      article in Article,
      preload: [:author, :mentions],
      where: article.id == ^id
    )
    |> filter_visible_articles(only_visible)
    |> Repo.one!()
  end

  @doc """
  Gets a single article by its slug.

  Raises `Ecto.NoResultsError` if the Article does not exist.

  ## Examples

      iex> get_article_by_slug!("foobar")
      %Article{}

      iex> get_article_by_slug!("")
      ** (Ecto.NoResultsError)

  """
  def get_article_by_slug!(slug, only_visible \\ true) do
    from(
      article in Article,
      preload: [:author, :mentions],
      where: article.slug == ^slug
    )
    |> filter_visible_articles(only_visible)
    |> Repo.one!()
  end

  @doc """
  Gets the last created article (ordered by `inserted_at`).

  Returns nil if none found.

  ## Examples

      iex> get_last_article()
      %Article{}

      iex> get_last_article()
      nil

  """
  def get_last_article(only_visible \\ true) do
    from(
      article in Article,
      preload: [:author, :mentions],
      order_by: [desc: article.inserted_at],
      limit: 1
    )
    |> filter_visible_articles(only_visible)
    |> Repo.one()
  end

  @doc """
  Creates a article.

  ## Examples

      iex> create_article(%{field: value})
      {:ok, %Article{}}

      iex> create_article(%{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def create_article(user, attrs \\ %{}) do
    slug = gen_slug(attrs["slug"])

    attrs =
      attrs
      |> Map.update("guid", "https://wwwtech.de/articles/" <> slug, fn _ -> "https://wwwtech.de/articles/" <> slug end)
      |> Map.update("slug", slug, fn _ -> slug end)

    %Article{author_id: user.id, article_format: "markdown"}
    |> Article.changeset(attrs)
    |> Repo.insert()
  end

  defp gen_slug(slug) do
    {{year, mon, _}, _} = :calendar.local_time()
    months = [nil, "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"]
    "#{year}/#{Enum.at(months, mon)}/#{slug}"
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
