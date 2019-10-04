defmodule Wwwtech.Repo.Migrations.CreateLikes do
  use Ecto.Migration

  def change do
    create table(:likes) do
      add :in_reply_to, :string, null: false
      add :posse, :boolean, default: false, null: false
      add :show_in_index, :boolean, default: false, null: false

      add :author_id, references(:authors, on_delete: :nothing), null: false

      timestamps()
    end

    create index(:likes, [:author_id])
  end
end
