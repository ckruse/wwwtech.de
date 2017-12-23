defmodule WwwtechWeb.MentionController do
  use WwwtechWeb.Web, :controller
  use WwwtechWeb.Web, :web_controller

  alias WwwtechWeb.Helpers.Paging

  alias Wwwtech.Mentions

  plug(:require_login)
  plug(:scrub_params, "mention" when action in [:create, :update])

  def index(conn, params) do
    number_of_mentions = Mentions.count_mentions()
    paging = Paging.paginate(number_of_mentions, page: params["p"])
    mentions = Mentions.list_mentions(limit: paging.params)
    render(conn, "index.html", paging: paging, mentions: mentions)
  end

  def edit(conn, %{"id" => id}) do
    mention = Mentions.get_mention!(id)
    changeset = Mentions.change_mention(mention)
    render(conn, "edit.html", mention: mention, changeset: changeset)
  end

  def update(conn, %{"id" => id, "mention" => mention_params}) do
    mention = Mentions.get_mention!(id)

    case Mentions.update_mention(mention, mention_params) do
      {:ok, _mention} ->
        conn
        |> put_flash(:info, "Mention has successfully been updated.")
        |> redirect(to: mention_path(conn, :index))

      {:error, %Ecto.Changeset{} = changeset} ->
        render(conn, "edit.html", mention: mention, changeset: changeset)
    end
  end

  def delete(conn, %{"id" => id}) do
    mention = Mentions.get_mention!(id)

    # Here we use delete! (with a bang) because we expect
    # it to always work (and if it does not, it will raise).
    Mentions.delete_mention(mention)

    conn
    |> put_flash(:info, "Mention deleted successfully.")
    |> redirect(to: mention_path(conn, :index))
  end
end
