defmodule Wwwtech.Web do
  @moduledoc """
  A module that keeps using definitions for controllers,
  views and so on.

  This can be used in your application as:

      use Wwwtech.Web, :controller
      use Wwwtech.Web, :view

  The definitions below will be executed for every view,
  controller, etc, so keep them short and clean, focused
  on imports, uses and aliases.

  Do NOT define functions inside the quoted expressions
  below.
  """

  def model do
    quote do
      use Ecto.Schema
      use Timex

      import Ecto
      import Ecto.Changeset
      import Ecto.Query
    end
  end

  def controller do
    quote do
      use Phoenix.Controller

      alias Wwwtech.Repo
      import Ecto
      import Ecto.Query, only: [from: 1, from: 2]

      import Wwwtech.Router.Helpers
      import Wwwtech.Gettext
      import Wwwtech.Session, only: [current_user: 1, logged_in?: 1]

      import Wwwtech.AuthenticationPlug

      plug :store_user
    end
  end

  def view do
    quote do
      use Phoenix.View, root: "web/templates"

      # Import convenience functions from controllers
      import Phoenix.Controller, only: [get_csrf_token: 0, get_flash: 2, view_module: 1, action_name: 1]

      # Use all HTML functionality (forms, tags, etc)
      use Phoenix.HTML

      import Wwwtech.Router.Helpers
      import Wwwtech.ErrorHelpers
      import Wwwtech.Gettext
      import Wwwtech.Session, only: [current_user: 1, logged_in?: 1]

      import Scrivener.HTML

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
      import Wwwtech.Gettext
    end
  end

  @doc """
  When used, dispatch to the appropriate controller/view/etc.
  """
  defmacro __using__(which) when is_atom(which) do
    apply(__MODULE__, which, [])
  end
end
