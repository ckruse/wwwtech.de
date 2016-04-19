defmodule Wwwtech.Repo.Migrations.CreateLike do
  use Ecto.Migration

  def change do
    create table(:likes) do
      add :in_reply_to, :string, null: false
      add :author_id, references(:authors), null: false
      add :posse, :boolean, default: false, null: false

      timestamps
    end

  end
end
