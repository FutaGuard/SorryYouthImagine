use diesel::Queryable;
use diesel::Selectable;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct Users {
    id: Uuid,
    is_admin: bool, // user.is_admin can access manager panel
    active: bool,   // user.active can login, user can be deactivated by admin
}