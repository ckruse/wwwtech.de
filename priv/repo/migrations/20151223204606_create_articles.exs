defmodule Wwwtech.Repo.Migrations.CreateArticles do
  use Ecto.Migration

  def change do
    create table(:articles) do
      add :title, :string, null: false
      add :slug, :string, null: false
      add :lang, :string, null: false
      add :guid, :string, null: false
      add :article_format, :string, null: false, default: "markdown"
      add :in_reply_to, :string
      add :excerpt, :text
      add :body, :text, null: false
      add :published, :boolean, default: false, null: false
      add :posse, :boolean, default: false, null: false

      add :author_id, references(:authors, on_delete: :nothing), null: false

      timestamps()
    end

    create unique_index(:articles, [:slug])
    create unique_index(:articles, [:guid])

    create index(:articles, [:author_id])
  end
end
