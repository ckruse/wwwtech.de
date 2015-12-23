defmodule Wwwtech.Repo.Migrations.CreateNote do
  use Ecto.Migration

  def change do
    create table(:notes) do
      add :author_id, :integer, null: false
      add :content, :text, null: false
      add :in_reply_to, :string
      add :posse, :boolean, default: false, null: false

      timestamps
    end

  end
end
