defmodule Wwwtech.Repo.Migrations.AddAltToPictures do
  use Ecto.Migration

  def change do
    alter table(:pictures) do
      add :alt, :string
    end
  end
end
