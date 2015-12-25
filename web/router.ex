defmodule Wwwtech.Router do
  use Wwwtech.Web, :router

  pipeline :browser do
    plug :accepts, ["html", "atom"]
    plug :fetch_session
    plug :fetch_flash
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", Wwwtech do
    pipe_through :browser # Use the default browser stack

    get    "/login",  SessionController, :new
    post   "/login",  SessionController, :create
    delete "/logout", SessionController, :delete

    get "/", PageController, :index
    get "/software", PageController, :software
    get "/about", PageController, :about

    resources "/notes", NoteController
    get "/notes.atom", NoteController, :index_atom

    resources "/articles", ArticleController, except: [:show]
    get "/articles/:year/:mon/:slug", ArticleController, :show
    get "/articles.atom", ArticleController, :index_atom
  end

  # Other scopes may use custom stacks.
  # scope "/api", Wwwtech do
  #   pipe_through :api
  # end
end
