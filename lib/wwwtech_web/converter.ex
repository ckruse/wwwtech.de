defmodule WwwtechWeb.Converter do
  def to_html(object_or_string)

  def to_html(%{content: content}), do: to_html(content)
  def to_html(%{body: body}), do: to_html(body)

  def to_html(str) when is_bitstring(str) do
    html_doc = Earmark.as_html!(str, %Earmark.Options{code_class_prefix: "language-"})
    {:safe, html_doc}
  end

  def to_html(map, field),
    do: to_html(Map.get(map, field))
end
