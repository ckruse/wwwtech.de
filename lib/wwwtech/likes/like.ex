defmodule Wwwtech.Likes.Like do
  use Ecto.Schema
  import Ecto.Changeset

  schema "likes" do
    field :in_reply_to, :string
    field :posse, :boolean, default: false
    field :show_in_index, :boolean, default: false

    belongs_to(:author, Wwwtech.Accounts.Author)

    timestamps()
  end

  @doc false
  def changeset(like, attrs) do
    like
    |> cast(attrs, [:author_id, :in_reply_to, :posse, :show_in_index])
    |> validate_required([:author_id, :in_reply_to, :posse, :show_in_index])
  end
end
