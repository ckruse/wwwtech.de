defmodule Wwwtech.Repo.Migrations.CreatePictures do
  use Ecto.Migration

  def change do
    create table(:pictures) do
      add :title, :string
      add :lang, :string
      add :content, :text, null: false
      add :posse, :boolean, default: false, null: false
      add :in_reply_to, :string
      add :image_file_name, :string, null: false
      add :image_content_type, :string, null: false
      add :image_file_size, :integer, null: false
      add :image_updated_at, :naive_datetime
      add :show_in_index, :boolean, default: false, null: false

      add :author_id, references(:authors, on_delete: :nothing), null: false

      timestamps()
    end

    create index(:pictures, [:author_id])
  end
end
