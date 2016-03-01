defmodule Wwwtech.MentionController do
  use Wwwtech.Web, :controller
  use Wwwtech.Web, :web_controller

  alias Wwwtech.Mention

  plug :require_login
  plug :scrub_params, "mention" when action in [:create, :update]

  def index(conn, _params) do
    mentions = Repo.all(Mention)
    render(conn, "index.html", mentions: mentions)
  end

  def edit(conn, %{"id" => id}) do
    mention = Repo.get!(Mention, id)
    changeset = Mention.changeset(mention)
    render(conn, "edit.html", mention: mention, changeset: changeset)
  end

  def update(conn, %{"id" => id, "mention" => mention_params}) do
    mention = Repo.get!(Mention, id)
    changeset = Mention.changeset(mention, mention_params)

    case Repo.update(changeset) do
      {:ok, mention} ->
        conn
        |> put_flash(:info, "Mention updated successfully.")
        |> redirect(to: mention_path(conn, :show, mention))
      {:error, changeset} ->
        render(conn, "edit.html", mention: mention, changeset: changeset)
    end
  end

  def delete(conn, %{"id" => id}) do
    mention = Repo.get!(Mention, id)

    # Here we use delete! (with a bang) because we expect
    # it to always work (and if it does not, it will raise).
    Repo.delete!(mention)

    conn
    |> put_flash(:info, "Mention deleted successfully.")
    |> redirect(to: mention_path(conn, :index))
  end
end
