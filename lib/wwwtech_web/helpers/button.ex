defmodule WwwtechWeb.Helpers.Button do
  use Phoenix.HTML

  def btn_button(opts, do: contents) do
    {to, form, opts} = extract_button_options(opts)

    form_tag to, form do
      Phoenix.HTML.Form.submit(opts, do: contents)
    end
  end

  def btn_button(text, opts) do
    {to, form, opts} = extract_button_options(opts)

    form_tag to, form do
      Phoenix.HTML.Form.submit(text, opts)
    end
  end

  defp extract_button_options(opts) do
    {to, opts} = pop_required_option!(opts, :to, "option :to is required in button/2")
    {method, opts} = Keyword.pop(opts, :method, :post)

    {form, opts} = form_options(opts, method, "button")

    {to, form, opts}
  end

  defp pop_required_option!(opts, key, error_message) do
    {value, opts} = Keyword.pop(opts, key)

    unless value do
      raise ArgumentError, error_message
    end

    {value, opts}
  end

  defp form_options(opts, method, class) do
    {form, opts} = Keyword.pop(opts, :form, [])

    form =
      form
      |> Keyword.put_new(:class, class)
      |> Keyword.put_new(:method, method)
      |> Keyword.put_new(:enforce_utf8, false)

    {form, opts}
  end
end
