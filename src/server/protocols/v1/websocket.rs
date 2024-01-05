use crate::server::protocols::v1::{
    packages,
    structures::{
        AnsPackage, ApiNeopixel, ApiPwmChannelValue, ApiPwmEnable, ApiPwmFrequency, ApiUserLed,
    },
};
use actix::{Actor, Addr, AsyncContext, Handler, Message, StreamHandler};
use actix_web::HttpRequest;
use actix_web_actors::ws;
use lazy_static::lazy_static;
use paperclip::actix::{
    api_v2_operation, get,
    web::{self, HttpResponse},
    Apiv2Schema,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

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
                let v: Vec<&str> = text.split("&&").collect();

                for request in v {
                    let request = request.trim();
                    if request.starts_with('/') {
                        ctx.text(request_endpoint(request));
                    } else {
                        let error_msg = format!(
                            "{} {}, missing / ?",
                            json!("Error: Invalid command:"),
                            request
                        );
                        ctx.text(error_msg)
                    }
                }
            }
            Ok(ws::Message::Close(msg)) => ctx.close(msg),
            _ => (),
        }
    }
}

#[api_v2_operation(skip)]
#[get("/ws")]
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

#[derive(Deserialize, Apiv2Schema)]
pub struct WebsocketQuery {
    /// Regex filter to select the desired incoming messages
    filter: Option<String>,
}

fn request_endpoint(request: &str) -> String {
    let v: Vec<&str> = request.trim_start_matches('/').splitn(5, '/').collect();
    match v[0] {
        "input" => {
            let _package = packages::reading(packages::Sensors::from_str(v[1]).unwrap(), false);
            json!("Ok: Command received").to_string()
        }
        "output" => match v[1] {
            "userled" => {
                if v.len() == 3 {
                    let _package = packages::get_led_all();
                    json!("Ok: Command received").to_string()
                } else if v.len() == 4 {
                    match serde_json::from_str::<ApiUserLed>(v[2]) {
                        Ok(data) => {
                            let _package = packages::set_led(data.userled, data.value);
                            json!("Ok: Command received").to_string()
                        }
                        Err(err) => json!(format!(
                            "Error: JSON was not well-formatted. Details: {}",
                            err
                        ))
                        .to_string(),
                    }
                } else {
                    json!("Error: Invalid command selected").to_string()
                }
            }
            "neopixel" => {
                if v.len() == 3 {
                    match serde_json::from_str::<ApiNeopixel>(v[2]) {
                        Ok(data) => {
                            let _package =
                                packages::set_neopixel(vec![[data.red, data.green, data.blue]]);
                            json!("Ok: Command received").to_string()
                        }
                        Err(err) => json!(format!(
                            "Error: JSON was not well-formatted. Details: {}",
                            err
                        ))
                        .to_string(),
                    }
                } else {
                    json!("Error: Invalid command selected").to_string()
                }
            }
            "pwm" => match v[2] {
                "enable" => {
                    let _package: AnsPackage;
                    if v.len() == 4 {
                        match serde_json::from_str::<ApiPwmEnable>(v[3]) {
                            Ok(data) => {
                                let _package = packages::pwm_enable(data.enable);
                                json!("Ok: Command received").to_string()
                            }
                            Err(err) => json!(format!(
                                "Error: JSON was not well-formatted. Details: {}",
                                err
                            ))
                            .to_string(),
                        }
                    } else {
                        json!("Error: Invalid command selected").to_string()
                    }
                }

                "frequency" => {
                    let _package: AnsPackage;
                    if v.len() == 4 {
                        match serde_json::from_str::<ApiPwmFrequency>(v[3]) {
                            Ok(data) => {
                                let _package = packages::set_pwm_freq_hz(data.frequency);
                                json!("Ok: Command received").to_string()
                            }
                            Err(err) => json!(format!(
                                "Error: JSON was not well-formatted. Details: {}",
                                err
                            ))
                            .to_string(),
                        }
                    } else {
                        json!("Error: Invalid command selected").to_string()
                    }
                }
                _ => {
                    let _package: AnsPackage;
                    if v.len() == 5 {
                        match serde_json::from_str::<ApiPwmChannelValue>(v[4]) {
                            Ok(data) => {
                                let _package =
                                    packages::pwm_channel_value(data.channel, data.value);
                                json!("Ok: Command received").to_string()
                            }
                            Err(err) => json!(format!(
                                "Error: JSON was not well-formatted. Details: {}",
                                err
                            ))
                            .to_string(),
                        }
                    } else {
                        json!("Error: Invalid command selected").to_string()
                    }
                }
            },
            "get_connected" => json!(MANAGER.lock().unwrap().get_client_count()).to_string(),
            _ => json!("Error: Invalid command selected").to_string(),
        },
        _ => format!("{} {}", json!("Error: Invalid command:"), request),
    }
}
