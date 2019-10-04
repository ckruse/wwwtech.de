defmodule Wwwtech.Support.Factory do
  alias Wwwtech.Repo

  def build(:author) do
    id = System.unique_integer()

    %Wwwtech.Accounts.Author{
      name: "Author #{id}",
      email: "author-#{id}@example.org",
      avatar: "avatar-#{id}",
      encrypted_password: "foo"
    }
  end

  def build(:note) do
    %Wwwtech.Notes.Note{
      title: "Just a test",
      content: "This ist just a test",
      lang: "en",
      author: build(:author),
      show_in_index: true,
      note_type: "note"
    }
  end

  def build(:article) do
    id = System.unique_integer()
    date = Date.utc_today()

    %Wwwtech.Articles.Article{
      article_format: "markdown",
      body: "Just a test entry",
      guid: "guid-#{id}",
      lang: "en",
      slug: "#{date.year}/#{date.month}/just-a-test-entry-#{id}",
      title: "Just a test entry #{id}",
      author: build(:author)
    }
  end

  def build(:picture) do
    id = System.unique_integer()

    %Wwwtech.Pictures.Picture{
      title: "Picture #{id}",
      lang: "en",
      content: "Just a test",
      author: build(:author),
      image_file_name: "image-#{id}.png",
      image_content_type: "image/png",
      image_file_size: 0,
      image_updated_at: NaiveDateTime.utc_now() |> NaiveDateTime.truncate(:second)
    }
  end

  def build(:like) do
    id = System.unique_integer()

    %Wwwtech.Likes.Like{
      in_reply_to: "https://example.com/#{id}",
      author: build(:author)
    }
  end

  def build(:mention) do
    id = System.unique_integer()

    %Wwwtech.Mentions.Mention{
      source_url: "http://example.org/source/#{id}",
      target_url: "http://example.org/target/#{id}",
      author: "Author #{id}",
      mention_type: "reply"
    }
  end

  def build(factory_name, attributes) do
    factory_name |> build() |> struct(attributes)
  end

  def insert!(factory_name, attributes \\ []) do
    Repo.insert!(build(factory_name, attributes))
  end
end
