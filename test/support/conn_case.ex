defmodule WwwtechWeb.ConnCase do
  @moduledoc """
  This module defines the test case to be used by
  tests that require setting up a connection.

  Such tests rely on `Phoenix.ConnTest` and also
  import other functionality to make it easier
  to build common datastructures and query the data layer.

  Finally, if the test case interacts with the database,
  it cannot be async. For this reason, every test runs
  inside a transaction which is reset at the beginning
  of the test unless the test case is marked as async.
  """

  use ExUnit.CaseTemplate

  using do
    quote do
      # Import conveniences for testing with connections
      use Phoenix.ConnTest
      import WwwtechWeb.Router.Helpers

      import Plug.Test

      # The default endpoint for testing
      @endpoint WwwtechWeb.Endpoint

      def login(%Wwwtech.Accounts.Author{} = author), do: login(build_conn(), author)

      def login(%Plug.Conn{} = conn, author) do
        conn
        |> init_test_session(current_user: author.id)
      end
    end
  end

  setup tags do
    :ok = Ecto.Adapters.SQL.Sandbox.checkout(Wwwtech.Repo)

    unless tags[:async] do
      Ecto.Adapters.SQL.Sandbox.mode(Wwwtech.Repo, {:shared, self()})
    end

    {:ok, conn: Phoenix.ConnTest.build_conn()}
  end
end
