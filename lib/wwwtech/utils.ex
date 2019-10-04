defmodule Wwwtech.Utils do
  @spec blank?(any()) :: boolean()
  def blank?(nil), do: true
  def blank?(""), do: true
  def blank?(0), do: true
  def blank?(false), do: true
  def blank?([]), do: true
  def blank?(%Ecto.Association.NotLoaded{}), do: true
  def blank?(map) when map == %{}, do: true
  def blank?(_), do: false

  @spec present?(any()) :: boolean()
  def present?(v), do: not blank?(v)

  defguard is_present(v) when not is_nil(v) and v != "" and v != 0 and v != [] and v != %{} and v != false
  defguard is_blank(v) when is_nil(v) or v == "" or v == 0 or v == [] or v == %{} or v == false

  def logged_in?(conn),
    do: present?(conn.assigns[:current_user])

  def put_if_blank(map, key, value) do
    if blank?(Map.get(map, key)),
      do: Map.put(map, key, value),
      else: map
  end
end
