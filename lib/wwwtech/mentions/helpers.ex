defmodule Wwwtech.Mentions.Helpers do
  import Wwwtech.Utils

  def validate_mention(conn, params) do
    cond do
      String.trim(to_string(params["target"])) == "" ->
        {:error, conn |> Plug.Conn.send_resp(400, "This is not the host you're looking for (target is blank)")}

      String.trim(to_string(params["source"])) == "" ->
        {:error, conn |> Plug.Conn.send_resp(400, "This is not the host you're looking for (source is blank)")}

      !valid_target?(conn, params["target"]) ->
        {:error,
         conn |> Plug.Conn.send_resp(400, "This is not the host you're looking for (host is not equal to my host)")}

      Webmentions.is_valid_mention(params["source"], params["target"]) ->
        true

      true ->
        {:error, conn |> Plug.Conn.send_resp(400, "This is not the host you're looking for (mention is not valid)")}
    end
  end

  defp valid_target?(conn, target) do
    target_uri = URI.parse(target)
    my_uri = URI.parse(WwwtechWeb.Router.Helpers.page_url(conn, :index))

    target_uri.host == my_uri.host
  end

  def values_from_remote(url) do
    response =
      [Tesla.Middleware.FollowRedirects]
      |> Tesla.client()
      |> Tesla.get(url)

    with {:ok, %Tesla.Env{status: code} = response} when code in 200..299 <- response,
         %{"items" => items} = mf when items != [] <- Microformats2.parse(response.body, url) do
      item = List.first(mf["items"])

      %{
        "author" => get_value(item, "author"),
        "author_url" => get_sub_key(item, "author", "url"),
        "author_avatar" => get_sub_key(item, "author", "photo"),
        "title" => get_title(item, response.body),
        "excerpt" => get_excerpt(item),
        "mention_type" => guess_mention_type(item),
        "url" => get_url(url, mf)
      }
    else
      # we got a successful request but no microformats
      {:ok, %Tesla.Env{status: code} = response} when code in 200..299 -> get_values_from_html(url, response.body)
      _ -> %{}
    end
  end

  defp get_title(item, html) do
    case get_value(item, "name") do
      nil -> get_document_title(html)
      value -> value
    end
  end

  defp get_document_title(html) when is_binary(html),
    do: get_document_title(Floki.parse_document!(html))

  defp get_document_title(doc) do
    title = Floki.find(doc, "meta[property='og:title']") |> Floki.attribute("content") |> List.first()

    if present?(title),
      do: title,
      else: Floki.find(doc, "title") |> Floki.text()
  end

  defp get_url(url, mf) do
    if Regex.match?(~r/brid-gy.appspot.com/, url),
      do: List.first(mf["items"])["properties"]["url"] |> List.first(),
      else: url
  end

  defp get_value(item, key) do
    kv = (item["properties"][key] || []) |> List.first()

    cond do
      kv == nil -> nil
      is_bitstring(kv) -> kv
      true -> kv["value"]
    end
  end

  defp get_sub_key(item, key, subkey) do
    key_val = (item["properties"][key] || []) |> List.first()

    cond do
      is_map(key_val) and is_map(key_val["properties"]) ->
        (key_val["properties"][subkey] || []) |> List.first()

      true ->
        nil
    end
  end

  defp get_excerpt(item) do
    excerpt = (item["properties"]["content"] || []) |> List.first()

    cond do
      excerpt != nil and is_map(excerpt) -> excerpt["text"]
      excerpt != nil and is_bitstring(excerpt) -> excerpt
      excerpt != nil -> raise inspect(excerpt)
      true -> nil
    end
  end

  defp guess_mention_type(item) do
    cond do
      present?(get_value(item, "in-reply-to")) -> "reply"
      present?(get_value(item, "like-of")) -> "like"
      present?(get_value(item, "repost-of")) -> "repost"
      persontag?(item) -> "persontag"
      true -> nil
    end
  end

  defp persontag?(item) do
    present?(item["properties"]["category"]) &&
      (List.first(item["properties"]["category"])["type"] || []) |> List.first() == "h-card"
  end

  defp get_values_from_html(url, html) do
    case Floki.parse_document(html) do
      {:ok, doc} ->
        author = Floki.find(doc, "meta[name=author]") |> Floki.attribute("content") |> List.first()
        excerpt = Floki.find(doc, "meta[name=description]") |> Floki.attribute("content") |> List.first()
        title = get_document_title(doc)
        author_url = Floki.find(doc, "meta[property='og:article:author']") |> Floki.attribute("content") |> List.first()
        author_avatar = Floki.find(doc, "meta[property='og:image']") |> Floki.attribute("content") |> List.first()

        %{
          "author" => author,
          "author_url" => author_url,
          "author_avatar" => author_avatar,
          "title" => title,
          "excerpt" => excerpt,
          "mention_type" => "reply",
          "url" => url
        }

      _ ->
        %{}
    end
  end
end
