defmodule Wwwtech.Repo.Migrations.CreateNotes do
  use Ecto.Migration

  def change do
    create table(:notes) do
      add :title, :string, null: false
      add :lang, :string, null: false
      add :content, :text, null: false
      add :in_reply_to, :string
      add :posse, :boolean, default: false, null: false
      add :show_in_index, :boolean, default: false, null: false
      add :note_type, :string, null: false

      add :author_id, references(:authors, on_delete: :nothing), null: false

      timestamps()
    end

    create index(:notes, [:author_id])
  end
end
