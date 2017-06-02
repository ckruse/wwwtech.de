defmodule Wwwtech.Repo.Migrations.AddIndexToLikes do
  use Ecto.Migration

  def change do
    alter table(:likes) do
      add :show_in_index, :boolean, null: false, default: true
    end
  end
end
