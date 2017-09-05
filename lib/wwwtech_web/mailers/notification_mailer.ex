defmodule WwwtechWeb.NotificationMailer do
  use Bamboo.Phoenix, view: WwwtechWeb.NotificationMailerView

  def notify(mention) do
    new_email() |>
      from("cjk@defunct.ch") |>
      to("cjk@defunct.ch") |>
      subject("Neue Webmention") |>
      put_header("Errors-To", "cjk@defunct.ch") |>
      put_header("Return-Path", "cjk@defunct.ch") |>
      render("notify.text", mention: mention)
  end

end

# eof
