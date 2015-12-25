defmodule Wwwtech.Repo.Migrations.CreateArticle do
  use Ecto.Migration

  def change do
    create table(:articles) do
      add :author_id, :integer, null: false
      add :in_reply_to, :string
      add :title, :string, null: false
      add :slug, :string, null: false
      add :guid, :string, null: false
      add :article_format, :string, null: false, default: "markdown"
      add :excerpt, :text
      add :body, :text, null: false
      add :published, :boolean, null: false, default: false

      timestamps
    end
    create unique_index(:articles, [:slug])
    create unique_index(:articles, [:guid])

  end
end
