// @generated automatically by Diesel CLI.

diesel::table! {
    images (id) {
        id -> Uuid,
        date -> Nullable<Timestamp>,
        upload_by -> Nullable<Varchar>,
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

diesel::allow_tables_to_appear_in_same_query!(
    images,
    users,
);
