defmodule Wwwtech.Repo.Migrations.CreatePicture do
  use Ecto.Migration

  def change do
    create table(:pictures) do
      add :title, :string
      add :posse, :boolean, default: false
      add :author_id, references(:authors), null: false
      add :in_reply_to, :string
      add :image_file_name, :string
      add :image_content_type, :string
      add :image_file_size, :integer
      add :image_updated_at, :timestamp

      timestamps()
    end

  end
end
