defmodule Wwwtech.Repo do
  use Ecto.Repo, otp_app: :wwwtech
  use Scrivener, page_size: 20
end
