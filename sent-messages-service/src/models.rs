// message.rs
use chrono::NaiveDateTime; // Importa el tipo de fecha y hora
use diesel::prelude::*;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::schema;
use schema::messages; // Asegúrate de que el nombre del esquema sea correcto

#[derive(Queryable, Serialize, Debug)]
pub struct Message { 
    pub id: i32,
    pub body: String,
    pub typeM: String,
    pub datetime: NaiveDateTime,
    pub sender: String,
    pub receiver: String,
    pub context: Option<String>,
}

// #[derive(Insertable)]
// #[table_name = "messages"]
// pub struct NewMessage {
//     pub body: String,
//     pub typeM: String,
//     pub datetime: NaiveDateTime,
//     pub sender: String,
