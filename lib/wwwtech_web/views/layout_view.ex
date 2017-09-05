defmodule WwwtechWeb.LayoutView do
  use WwwtechWeb.Web, :view

  def page_title(conn, assigns) do
    try do
      apply(view_module(conn), :page_title, [action_name(conn), assigns]) <> " — WWWTech"
    rescue
      UndefinedFunctionError -> default_page_title(conn, assigns)
      FunctionClauseError -> default_page_title(conn, assigns)
    end
  end

  def default_page_title(_conn, _assigns) do
    "WWWTech — Free/Libre Open Source Software by Christian Kruse"
  end

  def description(conn, assigns) do
    try do
      apply(view_module(conn), :page_description, [action_name(conn), assigns])
    rescue
      UndefinedFunctionError -> default_page_description(conn, assigns)
      FunctionClauseError -> default_page_description(conn, assigns)
    end
  end

  def default_page_description(_conn, _assigns) do
    "Personal silo (Twitter, Facebook, …) replacement of Christian Kruse"
  end

  def distance_of_time_in_words(from_time, to_time) do
    difference = abs(Timex.diff(from_time, to_time, :seconds))

    case difference do
      # 0 <-> 29 secs => less than a minute
      x when x in 0..29 -> "less than a minute"

      # 30 secs <-> 1 min, 29 secs => 1 minute
      x when x in 30..89 -> "1 minute"

      # 1 min, 30 secs <-> 44 mins, 29 secs => [2..44] minutes
      x when x in 90..2669 -> Integer.to_string(trunc(Float.floor(x / 60))) <> " minutes"

      # 44 mins, 30 secs <-> 89 mins, 29 secs => about 1 hour
      x when x in 2670..5369 -> "about 1 hour"

      # 89 mins, 30 secs <-> 23 hrs, 59 mins, 29 secs => about [2..24] hours
      x when x in 5370..86369 -> "about " <> Integer.to_string(trunc(Float.floor(x / 3600))) <> " hours"

      # 23 hrs, 59 mins, 30 secs <-> 41 hrs, 59 mins, 29 secs => 1 day
      x when x in 86370..151169 -> "1 day"

      # 41 hrs, 59 mins, 30 secs  <-> 29 days, 23 hrs, 59 mins, 29 secs => [2..29] days
      x when x in 151170..2591969 -> Integer.to_string(trunc(Float.floor(x / 86400))) <> " days"

      # 29 days, 23 hrs, 59 mins, 30 secs <-> 44 days, 23 hrs, 59 mins, 29 secs   # => about 1 month
      x when x in 2591970..3887969 -> "about 1 month"

      # 44 days, 23 hrs, 59 mins, 30 secs <-> 59 days, 23 hrs, 59 mins, 29 secs => about 2 months
      x when x in 3887970..5183969 -> "about 2 months"

      # 59 days, 23 hrs, 59 mins, 30 secs <-> 1 yr minus 1 sec => [2..12] months
      x when x in 5183970..31535999 -> Integer.to_string(trunc(Float.floor(x / 2628000))) <> " months"

      # 1 yr <-> 1 yr, 3 months => about 1 year
      x when x in 31536000..39419999 -> "about 1 year"

      # 1 yr, 3 months <-> 1 yr, 9 months => over 1 year
      x when x in 39420000..55187999 -> "over 1 year"

      # 1 yr, 9 months <-> 2 yr minus 1 sec => almost 2 years
      x when x in 55188000..63072000 -> "almost 2 years"

      x -> Integer.to_string(trunc(Float.floor(x / 39420000))) <> " years"
    end
  end

  def time_ago_in_words(from_time) do
    distance_of_time_in_words(from_time, Timex.local) <> " ago"
  end

  def filtered_mentions(mentions, type \\ "reply") do
    Enum.filter(mentions, fn(el) -> el.mention_type == type end)
  end

  def link_class_by_type(type) do
    case type do
      "reply" ->
        "u-in-reply-to"
      "repost" ->
        "u-repost-of"
      _ ->
        ""
    end
  end

  def entry_class_by_type(type) do
    case type do
      "reply" ->
        "h-as-reply"
      "repost" ->
        "p-repost"
      "like" ->
        "p-like"
      "favorite" ->
        "p-favorite"
      "tag" ->
        "p-tag"
      "bookmark" ->
        "p-bookmark"
      _ ->
        ""
    end
  end

  def safe_html(str) do
    {_, data} = Phoenix.HTML.html_escape(str)
    data
  end
end
