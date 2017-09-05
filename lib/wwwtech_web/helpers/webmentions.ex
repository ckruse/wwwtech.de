defmodule WwwtechWeb.Helpers.Webmentions do
  def send_webmentions(article, url, type, action) do
    urls = if article.published do
        case Webmentions.send_webmentions(url) do
          {:ok, list} ->
            list
          _ ->
            []
        end
      else
        []
      end

    "#{type} #{action} successfully. Webmentions sent to these endpoints:\n" <> Webmentions.results_as_text(urls)
  end

  def send_webmentions(url, type, action) do
    urls = case Webmentions.send_webmentions(url) do
        {:ok, list} ->
          list
        _ ->
          []
      end

    "#{type} #{action} successfully. Webmentions sent to these endpoints:\n" <> Webmentions.results_as_text(urls)
  end
end
