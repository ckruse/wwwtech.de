defmodule Wwwtech.Repo.Migrations.CreateMentions do
  use Ecto.Migration

  def change do
    create table(:mentions) do
      add :source_url, :string, null: false
      add :target_url, :string, null: false
      add :title, :string
      add :excerpt, :string
      add :author, :string, null: false
      add :author_url, :string
      add :author_avatar, :string
      add :mention_type, :string, null: false

      add :note_id, references(:notes, on_delete: :nothing)
      add :article_id, references(:articles, on_delete: :nothing)
      add :picture_id, references(:pictures, on_delete: :nothing)

      timestamps()
    end

    create index(:mentions, [:note_id])
    create index(:mentions, [:article_id])
    create index(:mentions, [:picture_id])
  end
end
