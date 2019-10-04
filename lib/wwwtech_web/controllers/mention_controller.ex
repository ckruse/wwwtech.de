defmodule WwwtechWeb.MentionController do
  use WwwtechWeb, :controller

  alias Wwwtech.Mentions
  alias WwwtechWeb.Paging

  plug :require_login

  def index(conn, params) do
    number_of_mentions = Mentions.count_mentions()
    paging = Paging.paginate(number_of_mentions, page: params["p"])
    mentions = Mentions.list_mentions(limit: paging.limit, offset: paging.offset)

    render(conn, "index.html", mentions: mentions, paging: paging)
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
        |> redirect(to: Routes.mention_path(conn, :index))

      {:error, %Ecto.Changeset{} = changeset} ->
        render(conn, "edit.html", mention: mention, changeset: changeset)
    end
  end

  def delete(conn, %{"id" => id}) do
    mention = Mentions.get_mention!(id)
    {:ok, _mention} = Mentions.delete_mention(mention)

    conn
    |> put_flash(:info, "Mention deleted successfully.")
    |> redirect(to: Routes.mention_path(conn, :index))
  end
end
