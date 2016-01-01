defmodule Wwwtech.WebmentionController do
  use Wwwtech.Web, :controller

  alias Wwwtech.Note
  alias Wwwtech.Picture
  alias Wwwtech.Article
  alias Wwwtech.Mention

  def mention(conn, params) do
    case is_valid_mention(conn, params) do
      :ok ->
        {object, type} = case Regex.run(~r/(notes|pictures|articles)\/(.*)/, params["target"]) do
                           [_, module, id] ->
                             {object_by_module(module, id), module}
                           _val ->
                             {nil, nil}
                         end

        if object == nil do
          conn |> send_resp 400, "This is not the host you're looking for (object could not be found)"
          raise :return
        end

        case Mention |> Mention.by_source_and_target(params["source"], params["target"]) |> Repo.one do
          nil ->
            changeset =  case type do
                           "notes" ->
                             Mention.changeset(%Mention{}, %{source_url: params["source"],
                                                             target_url: params["target"],
                                                             note_id: object.id,
                                                             mention_type: "mention",
                                                             author: ""})

                           "pictures" ->
                             Mention.changeset(%Mention{}, %{source_url: params["source"],
                                                             target_url: params["target"],
                                                             picture_id: object.id,
                                                             mention_type: "mention",
                                                             author: ""})

                           "articles" ->
                             Mention.changeset(%Mention{}, %{source_url: params["source"],
                                                             target_url: params["target"],
                                                             article_id: object.id,
                                                             mention_type: "mention",
                                                             author: ""})
                         end

            Repo.insert(changeset)

          _object ->
            nil
        end

        :gen_smtp_client.send({"cjk@defunct.ch", ["cjk@defunct.ch"],
                               "Subject: [WWWTech] New Mention\r\nFrom: Christian Kruse <cjk@defunct.ch>\r\nTo: Christian Kruse <cjk@defunct.ch>\r\n\r\nSource: #{params["source"]}\r\nTarget: #{params["target"]}"},
                              [{:relay, Application.get_env(:wwwtech, :smtp_server)},
                               {:username, to_char_list(Application.get_env(:wwwtech, :smtp_user))},
                               {:password, to_char_list(Application.get_env(:wwwtech, :smtp_password))}])

        conn |> send_resp 201, "Accepted"
      {:error, nconn} ->
        nconn
    end
  end

  def is_valid_mention(conn, params) do
    if String.strip(to_string(params["target"])) == "" do
      {:error, (conn |> send_resp 400, "This is not the host you're looking for (target is blank)")}
    else
      if String.strip(to_string(params["source"])) == "" do
        {:error, (conn |> send_resp 400, "This is not the host you're looking for (source is blank)")}
      else
        target_uri = URI.parse(params["target"])
        my_uri = URI.parse(page_url(conn, :index))

        if target_uri.host != my_uri.host do
          {:error, (conn |> send_resp 400, "This is not the host you're looking for (host is not equal to my host)")}
        else
          if Wwwtech.Webmentions.is_valid_mention(params["source"], params["target"]) do
            :ok
          else
            {:error, (conn |> send_resp 400, "This is not the host you're looking for (mention is not valid)")}
          end
        end
      end
    end
  end

  def object_by_module(module, id) do
    case module do
      "articles" ->
        Article |>
          Article.by_slug(id) |>
          Article.only_visible(false) |>
          Repo.one!
      "notes" ->
        Repo.get!(Note, id)
      "pictures" ->
        Repo.get!(Picture, id)
      _ ->
        nil
    end
  end
end
