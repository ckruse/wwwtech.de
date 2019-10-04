defmodule Wwwtech.Accounts.Author do
  use Ecto.Schema
  import Ecto.Changeset

  schema "authors" do
    field :avatar, :string
    field :email, :string
    field :encrypted_password, :string
    field :password, :string, virtual: true
    field :name, :string

    timestamps()
  end

  @doc false
  def changeset(author, attrs) do
    author
    |> cast(attrs, [:name, :email, :avatar, :password])
    |> validate_required([:name, :email, :avatar])
    |> put_password_hash()
  end

  defp put_password_hash(%Ecto.Changeset{valid?: true, changes: %{password: password}} = changeset),
    do: change(changeset, Argon2.add_hash(password, hash_key: :encrypted_password))

  defp put_password_hash(changeset), do: changeset
end
