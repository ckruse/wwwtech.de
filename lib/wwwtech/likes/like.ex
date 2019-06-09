defmodule Wwwtech.Likes.Like do
  use Ecto.Schema
  import Ecto.Changeset
  alias Wwwtech.Likes.Like

  schema "likes" do
    field(:in_reply_to, :string, null: false)
    field(:posse, :boolean, default: false, null: false)
    field(:show_in_index, :boolean, default: true, null: false)
    belongs_to(:author, Wwwtech.Accounts.Author)

    timestamps()
  end

  @doc """
  Creates a changeset based on the `model` and `params`.

  If no params are provided, an invalid changeset is returned
  with no validation performed.
  """
  def changeset(%Like{} = like, params \\ %{}) do
    like
    |> cast(params, [:author_id, :in_reply_to, :posse, :show_in_index])
    |> validate_required([:author_id, :in_reply_to, :posse, :show_in_index])
  end
end
