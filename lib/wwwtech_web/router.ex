defmodule WwwtechWeb.Router do
  use WwwtechWeb.Web, :router

  pipeline :browser do
    plug(:accepts, ["html", "atom"])
    plug(:fetch_session)
    plug(:fetch_flash)
    plug(:protect_from_forgery)
    plug(:put_secure_browser_headers)
    plug(WwwtechWeb.Plug.CurrentUser)
    plug(WwwtechWeb.Plug.RememberMe)
  end

  pipeline :api do
    plug(:accepts, ["json", "html"])
  end

  scope "/", WwwtechWeb do
    # Use the default browser stack
    pipe_through(:browser)

    get("/login", SessionController, :new)
    post("/login", SessionController, :create)
    delete("/logout", SessionController, :delete)

    get("/", PageController, :index)
    get("/software", PageController, :software)
    get("/about", PageController, :about)
    get("/whatsnew.atom", PageController, :index_atom)
    get("/more", PageController, :more)
    get("/.well-known/keybase.txt", PageController, :keybase)

    resources("/notes", NoteController)
    get("/notes.atom", NoteController, :index_atom)

    resources("/articles", ArticleController, except: [:show])
    get("/articles/:year/:mon/:slug", ArticleController, :show)
    get("/articles.atom", ArticleController, :index_atom)

    resources("/pictures", PictureController)
    get("/pictures.atom", PictureController, :index_atom)

    resources("/likes", LikeController)
    get("/likes.atom", LikeController, :index_atom)

    resources("/mentions", MentionController, except: [:create, :new, :show])

    get("/cache", CacheController, :show)
  end

  scope "/", WwwtechWeb do
    pipe_through(:api)

    post("/webmentions", WebmentionController, :mention)
  end

  # Other scopes may use custom stacks.
  # scope "/api", Wwwtech do
  #   pipe_through :api
  # end
end
