defmodule Wwwtech.DeployTask do
  alias Wwwtech.Utils

  def deploy(conn, payload) when is_binary(payload),
    do: deploy(conn, Jason.decode!(payload))

  def deploy(_conn, %{"action" => "published", "release" => %{"html_url" => url}}) do
    version = Regex.replace(~r{.*/}, url, "")
    script = Application.get_env(:wwwtech, :deploy_script)

    if Utils.present?(script) do
      Task.start(fn ->
        Process.sleep(10000)
        System.cmd(script, [version])
      end)
    end
  end

  def deploy(_conn, _),
    do: nil
end
