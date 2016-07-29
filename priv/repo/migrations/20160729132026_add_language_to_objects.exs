defmodule Wwwtech.Repo.Migrations.AddLanguageToObjects do
  use Ecto.Migration

  def change do
    alter table(:articles) do
      add :lang, :string, null: false, default: "en"
    end

    alter table(:notes) do
      add :lang, :string, null: false, default: "en"
    end

    alter table(:pictures) do
      add :lang, :string, null: false, default: "en"
    end
  end
end
