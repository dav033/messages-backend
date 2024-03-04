use crate::schema::messages;
use diesel::Insertable;
use serde::Serialize;

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=messages)]
pub struct NewMessage {
    pub body: String,
    pub typeM: String,
    pub datetime: chrono::NaiveDateTime,
    pub sender: String,
    pub receiver: String,
    pub context: String,
}
