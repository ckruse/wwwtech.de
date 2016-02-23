defmodule Wwwtech.Repo.Migrations.AddTitleToNote do
  use Ecto.Migration

  def change do
    alter table(:notes) do
      add :title, :string, null: false
    end
  end

end
