defmodule WwwtechWeb.Api.LikeView do
  use WwwtechWeb, :view
  alias WwwtechWeb.Api.LikeView

  def render("index.json", %{likes: likes}) do
    render_many(likes, LikeView, "like.json")
  end

  def render("show.json", %{like: like}) do
    render_one(like, LikeView, "like.json")
  end

  def render("like.json", %{like: like}) do
    %{
      id: like.id,
      in_reply_to: like.in_reply_to,
      posse: like.posse,
      show_in_index: like.show_in_index,
      inserted_at: like.inserted_at |> Timex.local() |> Timex.format!("{ISO:Extended:Z}"),
      updated_at: like.updated_at |> Timex.local() |> Timex.format!("{ISO:Extended:Z}")
    }
  end
end
