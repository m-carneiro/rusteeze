use chrono::{NaiveDateTime, Utc};
use diesel::{Associations, Identifiable, Insertable, Queryable};
use crate::{
    config::db::Connection,
    models::user::User,
}

#[derive(Identifiable, Associations, Queryable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = login_history)]
pub struct LoginHistory {
    pub id: i32,
    pub username: String,
    pub login_timestamp: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = login_history)]
pub struct LoginHistoryInsertableDTO {
    pub user_id: i32,
    pub login_timestamp: NaiveDateTime,
}

impl LoginHistory {
    pub fn create(
        un: &str,
        conn: &mut Connection,
    ) -> Option<LoginHistoryInsertableDTO> {
        if let Ok(user) = User::find_user_by_username(un, conn) {
            let now = Utc::now();
            Some(LoginHistoryInsertableDTO {
                user_id: user.id,
                login_timestamp: now.naive_utc(),
            });
        } else {
            None
        }
    }
}

