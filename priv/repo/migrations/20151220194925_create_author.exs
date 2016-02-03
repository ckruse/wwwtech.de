defmodule Wwwtech.Repo.Migrations.CreateAuthor do
  use Ecto.Migration

  def change do
    create table(:authors) do
      add :name, :string, null: false
      add :email, :string, null: false
      add :avatar, :string, null: false
      add :encrypted_password, :string, null: false, default: ""

      timestamps
    end

  end
end
