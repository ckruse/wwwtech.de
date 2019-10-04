defmodule Wwwtech.EctoEnhancements do
  import Ecto.Query, warn: false
  import Wwwtech.Utils, only: [is_present: 1]

  def filter(query, field, value, binding_pos \\ 0)
  def filter(q, _, value, _) when not is_present(value), do: q
  def filter(q, field, %{id: id}, _), do: filter(q, field, id)

  def filter(q, field, value, binding_pos) when is_list(value),
    do: from([{rel, binding_pos}] in q, where: field(rel, ^field) in ^value)

  def filter(q, field, value, binding_pos),
    do: from([{rel, binding_pos}] in q, where: field(rel, ^field) == ^value)

  def apply_limit(query, limit, offset)

  def apply_limit(query, limit_val, offset_val) when is_number(limit_val) and is_number(offset_val) do
    query
    |> limit(^limit_val)
    |> offset(^offset_val)
  end

  def apply_limit(query, _, _), do: query

  def filter_hidden(query, visible_state, flag_field \\ :show_in_index)
  def filter_hidden(q, true, _), do: q
  def filter_hidden(q, _, flag_field), do: from(note in q, where: field(note, ^flag_field) == true)
end
