defmodule Wwwtech.FixArticleSlugs do
  import Ecto.Query, warn: false
  alias Wwwtech.Repo

  alias Wwwtech.Articles.Article

  def perform do
    Repo.transaction(fn ->
      from(article in Article)
      |> Repo.stream()
      |> Enum.each(&fix_slug/1)

      :ok
    end)
  end

  defp fix_slug(article) do
    [_, year_month, slug] = Regex.run(~r|^(\d+/\w+)/(.*)|, article.slug)
    wanted_year_month = Timex.format!(article.inserted_at, "%Y/%b", :strftime) |> String.downcase()

    if wanted_year_month != year_month do
      {1, nil} =
        from(article in Article, where: article.id == ^article.id)
        |> Repo.update_all(set: [slug: "#{wanted_year_month}/#{slug}"])
    end
  end
end
