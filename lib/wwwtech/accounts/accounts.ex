defmodule Wwwtech.Accounts do
  @moduledoc """
  The Accounts context.
  """

  import Ecto.Query, warn: false
  alias Wwwtech.Repo

  alias Wwwtech.Accounts.Author

  def get_author!(id), do: Repo.get!(Author, id)
  def get_author_by_email(email), do: Repo.get_by(Author, email: String.downcase(email))
end
