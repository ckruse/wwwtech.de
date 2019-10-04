defmodule WwwtechWeb.PictureView do
  use WwwtechWeb, :view

  def page_title(:index, _), do: "Pictures"
  def page_title(:show, assigns), do: "Picture #{assigns[:picture].id}: #{assigns[:picture].title}"

  def page_title(:new, _), do: "New Picture"
  def page_title(:create, _), do: "New Picture"

  def page_title(:edit, _), do: "Edit Picture"
  def page_title(:update, _), do: "Edit Picture"

  def picture_type(%{type: type}) when is_present(type), do: type
  def picture_type(_), do: "thumbnail"

  def picture_link(picture, %{type: :thumbnail, conn: conn}), do: Routes.picture_path(conn, :show, picture)
  def picture_link(picture, %{conn: conn}), do: picture_path_w_ct(conn, picture)

  def to_degrees(vals, ref) do
    [d, m, s] = vals
    degrees = d + m / 60.0 + s / 3600.0

    if ref != "N" and ref != "E" do
      0 - degrees
    else
      degrees
    end
  end

  def picture_path_w_ct(conn, picture),
    do: Routes.picture_path(conn, :show, picture) <> suffix(picture.image_content_type)

  def suffix("image/png"), do: ".png"
  def suffix("image/jpg"), do: ".jpg"
  def suffix("image/jpeg"), do: ".jpg"
  def suffix(_), do: ".unknown"

  def exif_date_time(exif) do
    exif.exif[:datetime_digitized]
    |> Timex.parse!("%Y:%m:%d %H:%M:%S", :strftime)
    |> Timex.format!("%A, %d. %B %Y %H:%M", :strftime)
  end

  def gps?(exif),
    do: present?(exif[:gps]) && present?(exif[:gps].gps_latitude) && present?(exif[:gps].gps_longitude)
end
