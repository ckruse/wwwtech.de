defmodule WwwtechWeb.WebmentionController do
  use WwwtechWeb, :controller

  alias Wwwtech.Notes
  alias Wwwtech.Pictures
  alias Wwwtech.Articles
  alias Wwwtech.Likes

  alias Wwwtech.Mentions
  alias Wwwtech.Mentions.Mention

  def create(conn, params) do
    case validate_mention(conn, params) do
      true -> save_mention(conn, params)
      {:error, nconn} -> nconn
    end
  end

  def save_mention(conn, params) do
    {object, type} =
      case Regex.run(~r/(notes|pictures|articles|likes)\/(.*)/, params["target"]) do
        [_, module, id] -> {object_by_module(module, id), module}
        _val -> {nil, nil}
      end

    if blank?(object) && present?(type) do
      conn |> send_resp(400, "This is not the host you're looking for (object could not be found)")
    else
      parsed = values_from_remote(params["source"])

      old_mention =
        case Mentions.get_mention_by_source_and_target(parsed["url"], params["target"]) do
          nil -> %Mention{}
          obj -> obj
        end

      attributes =
        %{
          source_url: parsed["url"],
          target_url: params["target"],
          mention_type: parsed["mention_type"] || "reply",
          author: parsed["author"],
          author_avatar: parsed["author_avatar"],
          author_url: parsed["author_url"],
          title: parsed["title"],
          excerpt: parsed["excerpt"]
        }
        |> object_id_with_key(type, object)

      is_new = old_mention.id == nil

      {:ok, mention} =
        if is_new do
          Mentions.create_mention(attributes)
        else
          Mentions.update_mention(old_mention, attributes)
        end

      if is_new || old_mention.title != attributes[:title] || old_mention.excerpt != attributes[:excerpt] do
        mention
        |> WwwtechWeb.NotificationMailer.notify()
        |> Wwwtech.Mailer.deliver_later()
      end

      conn |> send_resp(201, "Accepted")
    end
  end

  def validate_mention(conn, params) do
    cond do
      String.trim(to_string(params["target"])) == "" ->
        {:error, conn |> send_resp(400, "This is not the host you're looking for (target is blank)")}

      String.trim(to_string(params["source"])) == "" ->
        {:error, conn |> send_resp(400, "This is not the host you're looking for (source is blank)")}

      !valid_target?(conn, params["target"]) ->
        {:error, conn |> send_resp(400, "This is not the host you're looking for (host is not equal to my host)")}

      Webmentions.is_valid_mention(params["source"], params["target"]) ->
        true

      true ->
        {:error, conn |> send_resp(400, "This is not the host you're looking for (mention is not valid)")}
    end
  end

  defp valid_target?(conn, target) do
    target_uri = URI.parse(target)
    my_uri = URI.parse(Routes.page_url(conn, :index))

    target_uri.host == my_uri.host
  end

  defp object_id_with_key(attributes, "notes", object), do: Map.put(attributes, :note_id, object.id)
  defp object_id_with_key(attributes, "pictures", object), do: Map.put(attributes, :picture_id, object.id)
  defp object_id_with_key(attributes, "articles", object), do: Map.put(attributes, :article_id, object.id)
  defp object_id_with_key(attributes, "likes", object), do: Map.put(attributes, :like_id, object.id)
  defp object_id_with_key(attributes, _, _), do: attributes

  defp object_by_module("articles", id), do: Articles.get_article_by_slug!(id)
  defp object_by_module("notes", id), do: Notes.get_note!(id)
  defp object_by_module("pictures", id), do: Pictures.get_picture!(id)
  defp object_by_module("likes", id), do: Likes.get_like!(id)
  defp object_by_module(_, _), do: nil

  defp values_from_remote(url) do
    response = HTTPotion.get(url)

    if HTTPotion.Response.success?(response) do
      mf = Microformats2.parse(response.body, url)

      if Enum.count(mf[:items]) > 0 do
        item = List.first(mf[:items])

        %{
          "author" => get_value(item, :author),
          "author_url" => get_sub_key(item, :author, :url),
          "author_avatar" => get_sub_key(item, :author, :photo),
          "title" => get_value(item, :name),
          "excerpt" => get_excerpt(item),
          "mention_type" => guess_mention_type(item),
          "url" => get_url(url, mf)
        }
      else
        get_values_from_html(url, response.body)
      end
    else
      %{}
    end
  end

  def get_url(url, mf) do
    if Regex.match?(~r/brid-gy.appspot.com/, url),
      do: List.first(mf[:items])[:properties][:url] |> List.first(),
      else: url
  end

  defp get_value(item, key) do
    kv = (item[:properties][key] || []) |> List.first()

    cond do
      kv == nil -> nil
      is_bitstring(kv) -> kv
      true -> kv[:value]
    end
  end

  defp get_sub_key(item, key, subkey) do
    key_val = (item[:properties][key] || []) |> List.first()

    cond do
      is_map(key_val) and is_map(key_val[:properties]) ->
        (key_val[:properties][subkey] || []) |> List.first()

      true ->
        nil
    end
  end

  defp get_excerpt(item) do
    excerpt = (item[:properties][:content] || []) |> List.first()

    cond do
      excerpt != nil and is_map(excerpt) -> excerpt[:text]
      excerpt != nil and is_bitstring(excerpt) -> excerpt
      excerpt != nil -> raise inspect(excerpt)
      true -> nil
    end
  end

  defp guess_mention_type(item) do
    cond do
      present?(get_value(item, :in_reply_to)) -> "reply"
      present?(get_value(item, :like_of)) -> "like"
      present?(get_value(item, :repost_of)) -> "repost"
      persontag?(item) -> "persontag"
      true -> nil
    end
  end

  defp persontag?(item) do
    present?(item[:properties][:category]) &&
      (List.first(item[:properties][:category])[:type] || []) |> List.first() == "h-card"
  end

  defp get_values_from_html(url, html) do
    case Floki.parse_document(html) do
      {:ok, doc} ->
        author = Floki.find(doc, "meta[name=author]") |> Floki.attribute("content") |> List.first()
        excerpt = Floki.find(doc, "meta[name=description]") |> Floki.attribute("content") |> List.first()
        title = Floki.find(doc, "meta[property='og:title']") |> Floki.attribute("content") |> List.first()
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
