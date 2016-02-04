defmodule Wwwtech.Repo.Migrations.AddIndexToPicturesAndNotes do
  use Ecto.Migration

  def change do
    alter table(:notes) do
      add :show_in_index, :boolean, null: false, default: true
    end

    alter table(:pictures) do
      add :show_in_index, :boolean, null: false, default: true
    end
  end
end
