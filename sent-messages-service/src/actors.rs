use crate::db_messages::CreateMessage;
use crate::db_utils::DbActor;
use crate::insertables::NewMessage;
use crate::models::Message;
use crate::schema::messages::dsl::*;
use crate::schema::messages::id as message_id;

use actix::Handler;

use diesel::{self, prelude::*};

impl Handler<CreateMessage> for DbActor {
    type Result = QueryResult<Message>;

    fn handle(&mut self, msg: CreateMessage, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Create message: Error connecting to database");

        let new_message = NewMessage {
            body: msg.body,
            typeM: msg.typeM,
            datetime: msg.dateTime,
            sender: msg.sender,
            receiver: msg.receiver,
            context: msg.context,
        };

        let _ = diesel::insert_into(messages)
            .values(&new_message)
            .execute(&mut conn);

        let result: Result<Message, diesel::result::Error> =
            messages.order(message_id.desc()).first(&mut conn);

        result
    }
}
