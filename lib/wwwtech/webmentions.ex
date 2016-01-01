defmodule Wwwtech.WebmentionPlug do
  import Plug.Conn

  def send_webmentions(source_url) do
    response = HTTPotion.get(source_url)

    if HTTPotion.Response.success?(response) do
      document = Floki.parse(response.body)
      content = Floki.find(document, ".h-entry")
      links = Floki.find(content, "a[href]")

      for link <- links do
        dst = Floki.attribute(link, "href") |> List.first

        if Regex.match?(~r/https?:\/\//, dst) do
          case discover_endpoint(dst) do
            {:ok, nil} ->
              nil
            {:error, _} ->
              nil
            {:ok, endpoint} ->
              send_webmention(endpoint, source_url, dst)
          end
        end
      end

    else
      {:error, response.status_code}
    end
  end

  def send_webmention(endpoint, source, target) do
    response = HTTPotion.post(endpoint, [body: 'source=#{source}&target=#{target}',
                                         headers: ["Content-Type": "application/x-www-form-urlencoded"]])
    case HTTPotion.Response.success?(response) do
      true -> :ok
      _ -> :error
    end
  end

  def discover_endpoint(source_url) do
    response = HTTPotion.get(source_url)

    if HTTPotion.Response.success?(response) do
      if response.headers[:"Link"] != nil and is_webmention_link(response.headers[:"Link"]) do
        {:ok, Regex.replace(~r/^<|>$/, Regex.replace(~r/; rel=.*/, response.headers[:"Link"], ""), "")}
      else
        mention_link = Floki.parse(response.body) |>
          Floki.find("link") |>
          Enum.find(fn(x) ->
            is_webmention_link(Floki.attribute(x, "rel") |> List.first)
          end)

        if mention_link == nil do
          {:ok, nil}
        else
          {:ok, Floki.attribute(mention_link, "href") |> List.first}
        end
      end
    else
      {:error, response.status_code}
    end
  end

  def is_webmention_link(link) when is_binary(link) do
    Regex.match?(~r/http:\/\/webmention\/|webmention/, link)
  end

  def is_valid_mention(source_url, target_url) do
    response = HTTPotion.get(source_url)

    if HTTPotion.Response.success?(response) do
      Floki.parse(response.body) |>
        Floki.find("a, link") |>
        Enum.find(fn(x) ->
          {tagname, _, _} = x
          target = case tagname do
                     "a" ->
                       Floki.attribute(x, "href") |> List.first
                     _ ->
                       Floki.attribute(x, "rel") |> List.first
                   end

          target == target_url
        end) != nil
    else
      false
    end
  end

  def set_mention_header(conn, _opts \\ []) do
    conn |>
      put_resp_header("Link",
                      "<" <> Wwwtech.Router.Helpers.page_url(conn, :index) <> "webmentions>; rel=\"webmention\"")
  end
end

