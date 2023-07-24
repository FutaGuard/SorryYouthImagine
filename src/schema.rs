// @generated automatically by Diesel CLI.

diesel::table! {
    images (id) {
        id -> Uuid,
        date -> Timestamp,
        upload_by -> Nullable<Uuid>,
        md5 -> Nullable<Varchar>,
        origin_filename -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        is_admin -> Bool,
        active -> Bool,
    }
}

diesel::joinable!(images -> users (upload_by));

diesel::allow_tables_to_appear_in_same_query!(
    images,
    users,
);
