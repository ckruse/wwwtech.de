defmodule Wwwtech.Note do
  use Wwwtech.Web, :model

  schema "notes" do
    field :title, :string, null: false
    field :content, :string, null: false
    field :in_reply_to, :string
    field :posse, :boolean, default: false, null: false
    field :show_in_index, :boolean, default: true, null: false

    belongs_to :author, Wwwtech.Author

    timestamps([type: Ecto.DateTime, usec: false, inserted_at: :created_at, updated_at: :updated_at])
  end

  @required_fields ~w(author_id title content posse show_in_index)
  @optional_fields ~w(in_reply_to)

  @doc """
  Creates a changeset based on the `model` and `params`.

  If no params are provided, an invalid changeset is returned
  with no validation performed.
  """
  def changeset(model, params \\ :empty) do
    model
    |> cast(params, @required_fields, @optional_fields)
  end

  def only_index(query, logged_in) do
    if logged_in == true do
      query
    else
      query |> where(show_in_index: true)
    end
  end

  def with_author(query) do
    query
    |> preload([:author])
  end

  def sorted(query) do
    query
    |> order_by([n], desc: n.created_at)
  end

  def last_x(query, x) do
    query
    |> limit(^x)
  end

  def to_html(model) do
    Cmark.to_html model.content
  end

  def created_at_timex(note) do
    Ecto.DateTime.to_erl(note.created_at)
    |> Timex.Date.from
  end

  def updated_at_timex(note) do
    Ecto.DateTime.to_erl(note.created_at)
    |> Timex.Date.from
  end

  def today?(note) do
    Ecto.DateTime.to_date(note.created_at) == Ecto.Date.utc()
  end

  def yesterday?(note) do
    date = Ecto.Date.utc() |> Ecto.Date.to_erl |> Date.from |> Date.subtract(Time.to_timestamp(1, :days))
    ins_at = Ecto.Date.to_erl(Ecto.DateTime.to_date(note.created_at)) |> Timex.Date.from
    Date.equal?(date, ins_at)
  end
end
