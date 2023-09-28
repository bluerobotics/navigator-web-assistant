use actix::{Actor, Addr, AsyncContext, Handler, Message, StreamHandler};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use crate::server::protocols::v1::packages;

pub struct StringMessage(String);

impl Message for StringMessage {
    type Result = ();
}

#[derive(Serialize, Debug)]
pub struct WebsocketError {
    pub error: String,
}

#[derive(Debug)]
pub struct WebsocketActorContent {
    pub actor: Addr<WebsocketActor>,
    pub re: Option<Regex>,
}

#[derive(Debug, Default)]
pub struct WebsocketManager {
    pub clients: Vec<WebsocketActorContent>,
}

impl WebsocketManager {
    pub fn send(&self, value: &serde_json::Value, name: &str) {
        if self.clients.is_empty() {
            return;
        }

        let string = serde_json::to_string_pretty(value).unwrap();
        for client in &self.clients {
            let is_match = client.re.as_ref().map_or(false, |regx| regx.is_match(name));
            if is_match {
                client.actor.do_send(StringMessage(string.clone()));
            }
        }
    }
    pub fn get_client_count(&self) -> usize {
        self.clients.len()
    }
}

lazy_static! {
    static ref MANAGER: Arc<Mutex<WebsocketManager>> =
        Arc::new(Mutex::new(WebsocketManager::default()));
}

pub fn send_to_websockets(message: Value) {
    MANAGER.lock().unwrap().send(&message, &message.to_string());
}

#[derive(Debug)]
pub struct WebsocketActor {
    server: Arc<Mutex<WebsocketManager>>,
    pub filter: String,
}

impl WebsocketActor {
    pub fn new(message_filter: String) -> Self {
        Self {
            server: MANAGER.clone(),
            filter: message_filter,
        }
    }
}

impl Handler<StringMessage> for WebsocketActor {
    type Result = ();

    fn handle(&mut self, message: StringMessage, context: &mut Self::Context) {
        context.text(message.0);
    }
}

impl Actor for WebsocketActor {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebsocketActor {
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Starting websocket, add itself in manager.");
        self.server
            .lock()
            .unwrap()
            .clients
            .push(WebsocketActorContent {
                actor: ctx.address(),
                re: Regex::new(&self.filter).ok(),
            });
    }

    fn finished(&mut self, ctx: &mut Self::Context) {
        println!("Finishing websocket, remove itself from manager.");
        self.server
            .lock()
            .unwrap()
            .clients
            .retain(|x| x.actor != ctx.address());
    }

    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let text = text.trim();
                if text.starts_with('/') {
                    let v: Vec<&str> = text.splitn(5, '/').collect();
                    match v[1] {
                        "input" => {
                            let package =
                                packages::reading(packages::Sensors::from_str(v[2]).unwrap());
                            ctx.text(json!(package).to_string());
                        }
                        "get_connected" => {
                            ctx.text(
                                json!(self.server.lock().unwrap().get_client_count()).to_string(),
                            );
                        }
                        _ => ctx.text(json!("Error: Invalid command selected").to_string()),
                    }
                } else {
                    ctx.text(json!("Error: Invalid command").to_string())
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub async fn websocket(
    req: HttpRequest,
    query: web::Query<WebsocketQuery>,
    stream: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    let filter = match query.into_inner().filter {
        Some(filter) => filter,
        _ => ".*".to_owned(),
    };

    log::debug!("New websocket with filter {:#?}", &filter);

    ws::start(WebsocketActor::new(filter), &req, stream)
}

#[derive(Deserialize)]
pub struct WebsocketQuery {
    /// Regex filter to select the desired incoming messages
    filter: Option<String>,
}
