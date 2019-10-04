defmodule Wwwtech.Support.Helpers do
  def unload_relations(obj, to_remove \\ nil)

  def unload_relations(objects, to_remove) when is_list(objects) do
    objects
    |> Enum.map(&unload_relations(&1, to_remove))
    |> Enum.sort(fn a, b ->
      Map.get(a, List.first(a.__struct__.__schema__(:primary_key))) >=
        Map.get(b, List.first(b.__struct__.__schema__(:primary_key)))
    end)
  end

  def unload_relations(obj, to_remove) do
    assocs =
      if to_remove == nil,
        do: obj.__struct__.__schema__(:associations),
        else: Enum.filter(obj.__struct__.__schema__(:associations), &(&1 in to_remove))

    Enum.reduce(assocs, obj, fn assoc, obj ->
      assoc_meta = obj.__struct__.__schema__(:association, assoc)

      Map.put(obj, assoc, %Ecto.Association.NotLoaded{
        __field__: assoc,
        __owner__: assoc_meta.owner,
        __cardinality__: assoc_meta.cardinality
      })
    end)
  end

  def equal_objects(a, b),
    do: unload_relations(a) == unload_relations(b)

  def ids(objects) when is_list(objects) do
    objects
    |> Enum.map(&Map.get(&1, List.first(&1.__struct__.__schema__(:primary_key))))
    |> Enum.sort()
  end

  def attrs(obj) do
    obj
    |> Map.from_struct()
    |> Map.drop(obj.__struct__.__schema__(:associations))
    |> Map.drop([:__struct__])
  end
end
