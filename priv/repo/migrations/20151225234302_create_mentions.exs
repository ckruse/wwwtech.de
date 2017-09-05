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

      add :note_id, references(:notes)
      add :picture_id, references(:pictures)
      add :article_id, references(:articles)


      timestamps()
    end

  end
end
