defmodule WwwtechWeb do
  @moduledoc """
  The entrypoint for defining your web interface, such
  as controllers, views, channels and so on.

  This can be used in your application as:

      use WwwtechWeb, :controller
      use WwwtechWeb, :view

  The definitions below will be executed for every view,
  controller, etc, so keep them short and clean, focused
  on imports, uses and aliases.

  Do NOT define functions inside the quoted expressions
  below. Instead, define any helper function in modules
  and import those modules here.
  """

  def controller do
    quote do
      use Phoenix.Controller, namespace: WwwtechWeb

      import Plug.Conn
      import WwwtechWeb.Gettext
      import Wwwtech.Utils
      import WwwtechWeb.Plug.Webmention, only: [set_mention_header: 2]
      import WwwtechWeb.Plug.Caching, only: [set_caching_headers: 2]
      import WwwtechWeb.Plug.Authentication, only: [require_login: 2, require_logout: 2]
      alias WwwtechWeb.Router.Helpers, as: Routes
      alias WwwtechWeb.PathHelpers
    end
  end

  def view do
    quote do
      use Phoenix.View,
        root: "lib/wwwtech_web/templates",
        namespace: WwwtechWeb

      use Appsignal.Phoenix.View

      # Import convenience functions from controllers
      import Phoenix.Controller, only: [get_flash: 1, get_flash: 2, view_module: 1]

      # Use all HTML functionality (forms, tags, etc)
      use Phoenix.HTML

      import WwwtechWeb.ErrorHelpers
      import WwwtechWeb.Gettext
      import Wwwtech.Utils
      alias WwwtechWeb.Router.Helpers, as: Routes
      alias WwwtechWeb.PathHelpers
      alias WwwtechWeb.Paging
    end
  end

  def router do
    quote do
      use Phoenix.Router
      import Plug.Conn
      import Phoenix.Controller
    end
  end

  def channel do
    quote do
      use Phoenix.Channel
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
