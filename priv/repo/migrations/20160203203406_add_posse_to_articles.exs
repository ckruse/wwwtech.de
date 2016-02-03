defmodule Wwwtech.Repo.Migrations.AddPosseToArticles do
  use Ecto.Migration

  def change do
    alter table(:articles) do
      add :posse, :boolean, null: false, default: false
    end
  end
end
