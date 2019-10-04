defmodule Wwwtech.Accounts do
  @moduledoc """
  The Accounts context.
  """

  import Ecto.Query, warn: false
  alias Wwwtech.Repo

  alias Wwwtech.Accounts.Author

  @doc """
  Gets a single author.

  Raises `Ecto.NoResultsError` if the Author does not exist.

  ## Examples

      iex> get_author!(123)
      %Author{}

      iex> get_author!(456)
      ** (Ecto.NoResultsError)

  """
  @spec get_author!(term()) :: %Author{}
  def get_author!(id), do: Repo.get!(Author, id)

  @spec get_author_by_email(String.t()) :: %Author{} | nil
  def get_author_by_email(email), do: Repo.get_by(Author, email: String.downcase(email))

  def update_password(author, pass) do
    author
    |> Author.changeset(%{password: pass})
    |> Repo.update()
  end
end
