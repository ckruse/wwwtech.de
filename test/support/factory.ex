defmodule Wwwtech.Factory do
  use ExMachina.Ecto, repo: Wwwtech.Repo

  def author_factory do
    %Wwwtech.Accounts.Author{
      name: sequence("Author "),
      email: sequence(:email, &"email-#{&1}@example.com"),
      avatar: "https://example.com/foo-bar.png",
      encrypted_password: Comeonin.Bcrypt.hashpwsalt("abcd")
    }
  end

  def article_factory do
    %Wwwtech.Articles.Article{
      title: sequence("Article "),
      slug: sequence("2017/sep/article-"),
      guid: sequence("https://wwwtech.de/foo/bar-"),
      article_format: "markdown",
      body: "Just some random string",
      published: true,
      posse: false,
      author: build(:author),
      mentions: []
    }
  end

  def note_factory do
    %Wwwtech.Notes.Note{
      title: sequence("Note "),
      lang: "en",
      content: "Just some random string",
      show_in_index: true,
      posse: false,
      note_type: "note",
      author: build(:author),
      mentions: []
    }
  end

  def like_factory do
    %Wwwtech.Likes.Like{
      show_in_index: true,
      posse: false,
      in_reply_to: sequence("https://example.com/foo-"),
      author: build(:author)
    }
  end

  def mention_factory do
    %Wwwtech.Mentions.Mention{
      source_url: sequence("https://example.com/foo-"),
      target_url: sequence("https://example.com/bar-"),
      title: sequence("Mention "),
      author: "Luke",
      mention_type: "reply"
    }
  end
end
