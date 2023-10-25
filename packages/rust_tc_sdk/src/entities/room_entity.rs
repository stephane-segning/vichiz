use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[diesel(table_name = crate::services::schema::rooms)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RoomEntity {
  pub id: String,
  pub name: String,
}
