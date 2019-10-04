defmodule Wwwtech.Repo do
  use Ecto.Repo,
    otp_app: :wwwtech,
    adapter: Ecto.Adapters.Postgres

  def maybe_preload(rel, nil), do: rel
  def maybe_preload(rel, preloads), do: preload(rel, preloads)
end
