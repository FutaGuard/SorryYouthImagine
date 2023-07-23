use std::time::SystemTime;
use diesel::prelude::*;
use uuid::Uuid;

use serde::{Deserialize, Serialize};


#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct Users {
    pub id: Uuid,
    pub is_admin: bool, // user.is_admin can access manager panel
    pub active: bool,   // user.active can login, user can be deactivated by admin
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::images)]
pub struct Images {
    pub id: Uuid,         // auto increment
    pub date: SystemTime,
    pub upload_by: i32,   // relate to user id
    pub md5: String,      // file md5 hash
    pub origin_filename: String,
}
