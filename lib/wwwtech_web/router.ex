defmodule WwwtechWeb.Router do
  use WwwtechWeb, :router
  import Phoenix.LiveDashboard.Router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_flash
    plug :protect_from_forgery
    plug :put_secure_browser_headers, %{"permissions-policy" => "interest-cohort=()"}

    plug(WwwtechWeb.Plug.CurrentUser)
    plug(WwwtechWeb.Plug.RememberMe)
  end

  pipeline :logged_in do
    plug(WwwtechWeb.Plug.LoggedIn)
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  pipeline :authed_api do
    plug :accepts, ["json"]
    plug WwwtechWeb.Plug.BasicAuth
  end

  scope "/", WwwtechWeb do
    pipe_through :browser

    get "/login", SessionController, :new
    post "/login", SessionController, :create
    delete "/logout", SessionController, :delete

    get "/", PageController, :index
    get "/software", PageController, :software
    get "/about", PageController, :about
    get "/whatsnew.atom", PageController, :index_atom
    get "/more", PageController, :more
    get "/.well-known/keybase.txt", PageController, :keybase
    get "/humans.txt", PageController, :humans

    resources "/notes", NoteController
    get "/notes.atom", NoteController, :index_atom

    resources "/articles", ArticleController, except: [:show]
    get "/articles/:year/:mon/:slug", ArticleController, :show
    get "/articles.atom", ArticleController, :index_atom

    post "/pictures/:id/regen", PictureController, :regenerate
    get "/pictures/scrolling", PictureController, :index_scrolling
    resources "/pictures", PictureController
    get "/pictures.atom", PictureController, :index_atom

    resources "/likes", LikeController
    get "/likes.atom", LikeController, :index_atom

    resources "/mentions", MentionController, except: [:create, :new, :show]
  end

  scope "/", WwwtechWeb do
    pipe_through [:browser, :logged_in]
    live_dashboard "/dashboard", metrics: WwwtechWeb.Telemetry
  end

  scope "/", WwwtechWeb do
    pipe_through :api

    post "/webmentions", WebmentionController, :create
  end

  scope "/api", WwwtechWeb.Api, as: :api do
    pipe_through :authed_api
    resources "/notes", NoteController, except: [:new, :edit]
    resources "/pictures", PictureController, except: [:new, :edit]
    resources "/likes", LikeController, except: [:new, :edit]
  end
end
