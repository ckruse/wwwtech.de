defmodule Wwwtech.Repo.Migrations.AddContentToPictures do
  use Ecto.Migration

  def up do
    Ecto.Adapters.SQL.query(Wwwtech.Repo, "ALTER TABLE pictures ADD COLUMN content TEXT", [])
    Ecto.Adapters.SQL.query(Wwwtech.Repo, "UPDATE pictures SET content = title", [])
    Ecto.Adapters.SQL.query(Wwwtech.Repo, "ALTER TABLE pictures ALTER COLUMN content SET NOT NULL", [])
  end

  def down do
    alter table(:pictures) do
      remove :content
    end
  end
end
