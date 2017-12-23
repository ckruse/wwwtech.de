defmodule WwwtechWeb.Web do
  @moduledoc """
  A module that keeps using definitions for controllers,
  views and so on.

  This can be used in your application as:

      use WwwtechWeb.Web, :controller
      use WwwtechWeb.Web, :view

  The definitions below will be executed for every view,
  controller, etc, so keep them short and clean, focused
  on imports, uses and aliases.

  Do NOT define functions inside the quoted expressions
  below.
  """

  def controller do
    quote do
      use Phoenix.Controller, namespace: WwwtechWeb
      use Timex

      alias Wwwtech.Repo
      import Ecto
      import Ecto.Query, only: [from: 1, from: 2]

      import WwwtechWeb.Router.Helpers
      import WwwtechWeb.Gettext
      import Wwwtech.Accounts.Session, only: [current_user: 1, logged_in?: 1]
      import Wwwtech.WebmentionPlug, only: [set_mention_header: 2]
      import Wwwtech.CachingPlug
    end
  end

  def web_controller do
    quote do
      import Wwwtech.AuthenticationPlug
      plug(:store_user)
    end
  end

  def view do
    quote do
      use Phoenix.View,
        root: "lib/wwwtech_web/templates",
        namespace: WwwtechWeb

      # Import convenience functions from controllers
      import Phoenix.Controller, only: [get_csrf_token: 0, get_flash: 2, view_module: 1, action_name: 1]

      # Use all HTML functionality (forms, tags, etc)
      use Phoenix.HTML

      import WwwtechWeb.Router.Helpers
      import WwwtechWeb.ErrorHelpers
      import WwwtechWeb.Gettext
      import Wwwtech.Accounts.Session, only: [current_user: 1, logged_in?: 1]

      import WwwtechWeb.Helpers.Button
      import WwwtechWeb.Helpers.Paging

      use Timex
    end
  end

  def router do
    quote do
      use Phoenix.Router
    end
  end

  def channel do
    quote do
      use Phoenix.Channel

      alias Wwwtech.Repo
      import Ecto
      import Ecto.Query, only: [from: 1, from: 2]
      import WwwtechWeb.Gettext
    end
  end

  @doc """
  When used, dispatch to the appropriate controller/view/etc.
  """
  defmacro __using__(which) when is_atom(which) do
    apply(__MODULE__, which, [])
  end
end
