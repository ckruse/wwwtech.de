defmodule WwwtechWeb.Api.PictureView do
  use WwwtechWeb, :view
  alias WwwtechWeb.Api.PictureView

  def render("index.json", %{pictures: pictures}) do
    render_many(pictures, PictureView, "picture.json")
  end

  def render("show.json", %{picture: picture}) do
    render_one(picture, PictureView, "picture.json")
  end

  def render("picture.json", %{picture: picture}) do
    %{
      id: picture.id,
      title: picture.title,
      lang: picture.lang,
      content: picture.content,
      in_reply_to: picture.in_reply_to,
      posse: picture.posse,
      show_in_index: picture.show_in_index,
      inserted_at: picture.inserted_at |> Timex.local() |> Timex.format!("{ISO:Extended:Z}"),
      updated_at: picture.updated_at |> Timex.local() |> Timex.format!("{ISO:Extended:Z}"),
      picture_url: picture_url_w_ct(WwwtechWeb.Endpoint, picture)
    }
  end

  def picture_url_w_ct(conn, picture) do
    WwwtechWeb.Router.Helpers.picture_url(conn, :show, picture) <>
      WwwtechWeb.PictureView.suffix(picture.image_content_type)
  end
end
