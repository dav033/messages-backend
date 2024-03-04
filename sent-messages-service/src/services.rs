use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};

use actix::Addr;

use crate::{db_messages::CreateMessage, AppState, DbActor};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateMessageBody {
    pub body: String,
    pub typeM: String,
    pub sender: String,
    pub receiver: String,
    pub context: String,
}

#[post("/messages")]
pub async fn send_messages(state: Data<AppState>, body: Json<CreateMessageBody>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db
        .send(CreateMessage {
            body: body.body.clone(),
            typeM: body.typeM.clone(),
            sender: body.sender.clone(),
            context: body.context.clone(),
            receiver: body.receiver.clone(),
            dateTime: chrono::Utc::now().naive_utc(),
        })
        .await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to create article"),
    }
}
