use crate::models::Message;
use chrono::NaiveDateTime; // Importa el tipo de fecha y hora
use diesel::QueryResult;

use actix::Message as ActixMessage;

#[derive(ActixMessage)]
#[rtype(result = "QueryResult<Message>")]
pub struct CreateMessage {
    pub body: String,
    pub typeM: String,
    pub sender: String,
    pub receiver: String,
    pub context: String,
    pub dateTime: NaiveDateTime,
}

  