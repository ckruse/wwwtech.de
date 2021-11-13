table! {
    articles (id) {
        id -> Int4,
        author_id -> Int4,
        in_reply_to -> Nullable<Varchar>,
        title -> Varchar,
        slug -> Varchar,
        guid -> Varchar,
        article_format -> Varchar,
        excerpt -> Nullable<Text>,
        body -> Text,
        published -> Bool,
        inserted_at -> Timestamp,
        updated_at -> Timestamp,
        posse -> Bool,
        lang -> Varchar,
    }
}

table! {
    authors (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        avatar -> Varchar,
        encrypted_password -> Varchar,
        remember_created_at -> Nullable<Timestamp>,
        inserted_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    likes (id) {
        id -> Int4,
        in_reply_to -> Varchar,
        author_id -> Int4,
        posse -> Bool,
        inserted_at -> Timestamp,
        updated_at -> Timestamp,
        show_in_index -> Bool,
    }
}

table! {
    mentions (id) {
        id -> Int4,
        source_url -> Varchar,
        target_url -> Varchar,
        title -> Nullable<Varchar>,
        excerpt -> Nullable<Varchar>,
        author -> Varchar,
        author_url -> Nullable<Varchar>,
        author_avatar -> Nullable<Varchar>,
        mention_type -> Varchar,
        note_id -> Nullable<Int4>,
        picture_id -> Nullable<Int4>,
        inserted_at -> Timestamp,
        updated_at -> Timestamp,
        article_id -> Nullable<Int4>,
        articles_id -> Nullable<Int4>,
    }
}

table! {
    notes (id) {
        id -> Int4,
        author_id -> Int4,
        content -> Text,
        in_reply_to -> Nullable<Varchar>,
        webmentions_count -> Int4,
        inserted_at -> Timestamp,
        updated_at -> Timestamp,
        posse -> Bool,
        title -> Varchar,
        show_in_index -> Bool,
        lang -> Varchar,
        note_type -> Varchar,
    }
}

table! {
    pictures (id) {
        id -> Int4,
        author_id -> Int4,
        in_reply_to -> Nullable<Varchar>,
        webmentions_count -> Int4,
        image_file_name -> Varchar,
        image_content_type -> Varchar,
        image_file_size -> Int4,
        image_updated_at -> Timestamp,
        inserted_at -> Timestamp,
        updated_at -> Timestamp,
        title -> Varchar,
        posse -> Bool,
        show_in_index -> Bool,
        content -> Text,
        lang -> Varchar,
        alt -> Nullable<Varchar>,
    }
}

table! {
    schema_migrations (version) {
        version -> Int8,
        inserted_at -> Nullable<Timestamp>,
    }
}

joinable!(articles -> authors (author_id));
joinable!(likes -> authors (author_id));
joinable!(mentions -> articles (article_id));
joinable!(mentions -> notes (note_id));
joinable!(mentions -> pictures (picture_id));
joinable!(notes -> authors (author_id));
joinable!(pictures -> authors (author_id));

allow_tables_to_appear_in_same_query!(
    articles,
    authors,
    likes,
    mentions,
    notes,
    pictures,
    schema_migrations,
);
