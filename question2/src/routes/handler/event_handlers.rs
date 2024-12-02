use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use sea_orm::{Database, EntityTrait, JsonValue};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use sea_orm::prelude::Uuid;
use sea_orm::{EntityTrait, IntoActiveModel, Set};
use serde::{Deserialize, Serialize};
use sea_orm::ActiveModelTrait;


#[derive(Deserialize, Serialize)]
struct Event {
    event_type: String,
    data: JsonValue,
    timestamp: String,
}

pub struct MyWebSocket {
    db: sea_orm::DatabaseConnection,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            if let Ok(event) = serde_json::from_str::<Event>(&text) {
                let db = self.db.clone();
                actix_rt::spawn(async move {
                    let new_event = event::ActiveModel {
                        id: sea_orm::ActiveValue::Set(uuid::Uuid::new_v4()),
                        event_type: sea_orm::ActiveValue::Set(event.event_type),
                        data: sea_orm::ActiveValue::Set(event.data),
                        timestamp: sea_orm::ActiveValue::Set(chrono::Utc::now().naive_utc()),
                    };

                    let _ = event::Entity::insert(new_event).exec(&db).await;
                });
            }
        } else if let Ok(ws::Message::Close(_)) = msg {
            ctx.stop();
        }
    }
}
