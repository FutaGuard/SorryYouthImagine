// @generated automatically by Diesel CLI.

diesel::table! {
    images (id) {
        id -> Uuid,
        date -> Timestamp,
        upload_by -> Uuid,
        md5 -> Varchar,
        origin_filename -> Varchar,
    }
}

diesel::table! {
    settings (id) {
        id -> Uuid,
        guest -> Bool,
        sitename -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        is_admin -> Bool,
        active -> Bool,
        token -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(images -> users (upload_by));

diesel::allow_tables_to_appear_in_same_query!(
    images,
    settings,
    users,
);
