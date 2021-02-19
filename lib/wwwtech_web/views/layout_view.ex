defmodule WwwtechWeb.LayoutView do
  use WwwtechWeb, :view

  def page_title(conn, assigns) do
    try do
      apply(view_module(conn), :page_title, [Phoenix.Controller.action_name(conn), assigns]) <> " — WWWTech"
    rescue
      UndefinedFunctionError -> default_page_title(conn, assigns)
      FunctionClauseError -> default_page_title(conn, assigns)
    end
  end

  def body_id(conn, assigns) do
    try do
      [
        {:safe, " id=\""},
        apply(view_module(conn), :body_id, [Phoenix.Controller.action_name(conn), assigns]),
        {:safe, "\""}
      ]
    rescue
      UndefinedFunctionError -> ""
      FunctionClauseError -> ""
    end
  end

  def default_page_title(_conn, _assigns) do
    "WWWTech — Open Source Software by Christian Kruse"
  end

  def description(conn, assigns) do
    try do
      apply(view_module(conn), :page_description, [Phoenix.Controller.action_name(conn), assigns])
    rescue
      UndefinedFunctionError -> default_page_description(conn, assigns)
      FunctionClauseError -> default_page_description(conn, assigns)
    end
  end

  def default_page_description(_conn, _assigns) do
    "Personal silo (Twitter, Facebook, …) replacement of Christian Kruse"
  end

  def date_heading(date) do
    cond do
      Timex.to_date(date) == Timex.today() -> "Today"
      Timex.to_date(date) == Timex.shift(Timex.today(), days: -1) -> "Yesterday"
      true -> Timex.format!(date, "%Y-%m-%d", :strftime)
    end
  end

  def entry_class_by_type("reply"), do: "h-as-reply"
  def entry_class_by_type("repost"), do: "p-repost"
  def entry_class_by_type("like"), do: "p-like"
  def entry_class_by_type("favorite"), do: "p-favorite"
  def entry_class_by_type("tag"), do: "p-tag"
  def entry_class_by_type("bookmark"), do: "p-bookmark"
  def entry_class_by_type(_), do: ""

  def link_class_by_type("reply"), do: "u-in-reply-to"
  def link_class_by_type("repost"), do: "u-repost-of"
  def link_class_by_type(_), do: ""

  def distance_of_time_in_words(from_time, to_time) do
    difference = abs(Timex.diff(from_time, to_time, :seconds))

    case difference do
      # 0 <-> 29 secs => less than a minute
      x when x in 0..29 ->
        "less than a minute"

      # 30 secs <-> 1 min, 29 secs => 1 minute
      x when x in 30..89 ->
        "1 minute"

      # 1 min, 30 secs <-> 44 mins, 29 secs => [2..44] minutes
      x when x in 90..2669 ->
        Integer.to_string(trunc(Float.floor(x / 60))) <> " minutes"

      # 44 mins, 30 secs <-> 89 mins, 29 secs => about 1 hour
      x when x in 2670..5369 ->
        "about 1 hour"

      # 89 mins, 30 secs <-> 23 hrs, 59 mins, 29 secs => about [2..24] hours
      x when x in 5370..86369 ->
        "about " <> Integer.to_string(trunc(Float.floor(x / 3600))) <> " hours"

      # 23 hrs, 59 mins, 30 secs <-> 41 hrs, 59 mins, 29 secs => 1 day
      x when x in 86370..151_169 ->
        "1 day"

      # 41 hrs, 59 mins, 30 secs  <-> 29 days, 23 hrs, 59 mins, 29 secs => [2..29] days
      x when x in 151_170..2_591_969 ->
        Integer.to_string(trunc(Float.floor(x / 86400))) <> " days"

      # 29 days, 23 hrs, 59 mins, 30 secs <-> 44 days, 23 hrs, 59 mins, 29 secs   # => about 1 month
      x when x in 2_591_970..3_884_369 ->
        "about 1 month"

      # 44 days, 23 hrs, 59 mins, 30 secs <-> 59 days, 23 hrs, 59 mins, 29 secs => about 2 months
      x when x in 3_884_370..5_180_369 ->
        "about 2 months"

      # 59 days, 23 hrs, 59 mins, 30 secs <-> 1 yr minus 1 sec => [2..12] months
      x when x in 5_180_370..31_556_951 ->
        Integer.to_string(trunc(Float.floor(x / 2_628_000))) <> " months"

      # 1 yr <-> 1 yr, 3 months => about 1 year
      x when x in 31_556_952..39_221_999 ->
        "about 1 year"

      # 1 yr, 3 months <-> 1 yr, 9 months => over 1 year
      x when x in 39_222_000..55_123_199 ->
        "over 1 year"

      # 1 yr, 9 months <-> 2 yr minus 1 sec => almost 2 years
      x when x in 55_123_200..63_113_903 ->
        "almost 2 years"

      x ->
        Integer.to_string(trunc(Float.floor(x / 31_556_952))) <> " years"
    end
  end

  def time_ago_in_words(from_time) do
    distance_of_time_in_words(from_time, Timex.local()) <> " ago"
  end

  def time_ago_or_date(from_time) do
    if Timex.diff(Timex.now(), from_time, :years) > 1,
      do: Timex.format!(from_time, "%e. %B %Y", :strftime),
      else: time_ago_in_words(from_time)
  end

  def has_flash?(conn, key) do
    val = get_flash(conn, key)
    !is_nil(val) && val != ""
  end

  def og_image(conn) do
    cond do
      conn.assigns[:picture] ->
        WwwtechWeb.PictureView.picture_path_w_ct(conn, conn.assigns[:picture])

      true ->
        Routes.static_path(conn, "/images/christian-kruse.jpg")
    end
  end

  def answers(%Ecto.Association.NotLoaded{}), do: []

  def answers(mentions) do
    Enum.filter(mentions, &(&1.mention_type == "reply"))
  end

  def likes(%Ecto.Association.NotLoaded{}), do: []

  def likes(mentions) do
    Enum.filter(mentions, &(&1.mention_type != "reply"))
  end
end
