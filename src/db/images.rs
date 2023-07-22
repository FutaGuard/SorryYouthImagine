use std::time::SystemTime;
use diesel::Queryable;
use diesel::Selectable;
use uuid::Uuid;
use diesel::prelude::*;


#[derive(Queryable, Selectable)]
#[diesel(table_name = images)]
pub struct Images {
    id: Uuid,         // auto increment
    date: SystemTime,
    upload_by: i32,   // relate to user id
    md5: String,      // file md5 hash
    origin_filename: String,
}

