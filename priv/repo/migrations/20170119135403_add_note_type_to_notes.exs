defmodule Wwwtech.Repo.Migrations.AddNoteTypeToNotes do
  use Ecto.Migration

  def change do
    alter table(:notes) do
      add :note_type, :string, null: false, default: "note"
    end
  end
end
