defmodule WwwtechWeb.Atom do
  def to_atom(collection, callbacks) do
    collection
    |> entries(callbacks)
    |> head(callbacks, List.first(collection))
    |> XmlBuilder.generate()
  end

  def head(collection, callbacks, newest) do
    {:feed, %{"xml:lang" => "en-US", "xmlns" => "http://www.w3.org/2005/Atom"},
     [
       {:id, nil, callbacks[:id]},
       {:link, %{"rel" => "alternate", "type" => "text/html", "href" => callbacks[:alternate_url]}, []},
       {:link, %{"rel" => "self", "type" => "application/atom+xml", "href" => callbacks[:self_url]}, []},
       {:title, nil, callbacks[:title]},
       {:updated, nil, Timex.lformat!(newest.updated_at, "{RFC3339z}", "en")},
       {:author, nil,
        [
          {:name, nil, "Christian Kruse"},
          {:email, nil, "cjk@defunct.ch"},
          {:uri, nil, "https://wwwtech.de/about"}
        ]},
       collection
     ]}
  end

  def entries(collection, callbacks) do
    Enum.map(collection, fn entry ->
      {:entry, nil,
       [
         {:id, nil, callbacks[:entry_id].(entry)},
         {:published, nil, Timex.lformat!(entry.inserted_at, "{RFC3339z}", "en")},
         {:updated, nil, Timex.lformat!(entry.updated_at, "{RFC3339z}", "en")},
         {:link,
          %{
            "rel" => "alternate",
            "type" => "text/html",
            # Path.message_url(conn, :show, thread, thread.message)
            "href" => callbacks[:entry_url].(entry)
          }, []},
         {:title, nil, callbacks[:entry_title].(entry)},
         {:content, %{"type" => "html"}, callbacks[:entry_content].(entry)}
       ]}
    end)
  end
end
