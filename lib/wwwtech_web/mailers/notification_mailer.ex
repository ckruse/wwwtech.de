defmodule WwwtechWeb.NotificationMailer do
  use Phoenix.Swoosh, view: WwwtechWeb.NotificationMailerView

  def notify(mention) do
    new()
    |> from("cjk@defunct.ch")
    |> to("cjk@defunct.ch")
    |> subject("Neue Webmention")
    |> header("Errors-To", "cjk@defunct.ch")
    |> header("Return-Path", "cjk@defunct.ch")
    |> render_body("notify.text", mention: mention)
  end
end

# eof
