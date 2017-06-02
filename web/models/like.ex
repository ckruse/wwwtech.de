defmodule Wwwtech.Like do
  use Wwwtech.Web, :model

  schema "likes" do
    field :in_reply_to, :string, null: false
    field :posse, :boolean, default: false, null: false
    field :show_in_index, :boolean, default: true, null: false
    belongs_to :author, Wwwtech.Author

    timestamps()
  end

  @required_fields [:author_id, :in_reply_to, :posse, :show_in_index]
  @optional_fields []

  @doc """
  Creates a changeset based on the `model` and `params`.

  If no params are provided, an invalid changeset is returned
  with no validation performed.
  """
  def changeset(model, params \\ %{}) do
    model
    |> cast(params, @required_fields ++ @optional_fields)
    |> validate_required(@required_fields)
  end

  def with_author(query) do
    query
    |> preload([:author])
  end

  def sorted(query) do
    query
    |> order_by([l], desc: l.inserted_at)
  end

  def last_x(query, x) do
    query
    |> limit(^x)
  end

  def only_index(query, logged_in) do
    if logged_in == true do
      query
    else
      query |> where(show_in_index: true)
    end
  end
end
