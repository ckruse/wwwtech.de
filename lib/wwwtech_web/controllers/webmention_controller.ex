defmodule WwwtechWeb.WebmentionController do
  use WwwtechWeb, :controller

  alias Wwwtech.Notes
  alias Wwwtech.Pictures
  alias Wwwtech.Articles
  alias Wwwtech.Likes

  alias Wwwtech.Mentions
  alias Wwwtech.Mentions.Mention
  alias Wwwtech.Mentions.Helpers

  def create(conn, params) do
    case Helpers.validate_mention(conn, params) do
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
      parsed = Helpers.values_from_remote(params["source"])

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
        Task.start(fn ->
          mention
          |> WwwtechWeb.NotificationMailer.notify()
          |> Wwwtech.Mailer.deliver!()
        end)
      end

      conn |> send_resp(201, "Accepted")
    end
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
end
