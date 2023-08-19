use std::sync::Mutex;

use actix::{Actor, AsyncContext, Handler, Message, StreamHandler};
use actix_web::{
    get,
    web::{self, Payload},
    Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws::{self};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::server::{Client, State};

pub struct Ws {
    app_data: web::Data<Mutex<State>>,
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct WsMessage {
    cmd: String,
    payload: serde_json::Value,
    id: String,
}

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

impl Handler<WsMessage> for Ws {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        let string_cmd = msg.cmd.to_string();
        let string_payload = msg.payload.to_string();
        let string_id = msg.id.to_string();

        ctx.text(
            json!({ "cmd": string_cmd, "payload": string_payload, "id": string_id }).to_string(),
        );
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Ws {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(..)) => {}
            Ok(ws::Message::Pong(..)) => {}
            Ok(ws::Message::Continuation(..)) => {}
            Ok(ws::Message::Text(text)) => {
                let string_text = &text.to_string()[..];
                let msg: WsMessage = serde_json::from_str(string_text).unwrap();

                match msg.cmd.as_ref() {
                    "connect" => {
                        let client = Client {
                            id: msg.clone().id,
                            addr: ctx.address(),
                        };

                        self.app_data.lock().unwrap().clients.push(client);
                    }
                    _ => {
                        for client in self.app_data.lock().unwrap().clients.iter() {
                            if client.id != msg.id {
                                client.addr.do_send(msg.clone());
                            }
                        }
                    }
                }
            }
            Ok(ws::Message::Binary(..)) => {}
            Ok(ws::Message::Nop) => {}
            Ok(ws::Message::Close(..)) => {
                let address = ctx.address();

                self.app_data
                    .lock()
                    .expect("Error!")
                    .clients
                    .retain(|c| c.addr != address);
            }
            Err(err) => {
                println!("Received error message: {}", err)
            }
        }
    }

    // fn started(&mut self, ctx: &mut Self::Context) {
    //     let address = ctx.address();
    //     ctx.text(format!("someone joined: {:?}", address));
    //
    //     self.app_data.lock().unwrap().clients.push(address);
    // }

    fn finished(&mut self, ctx: &mut Self::Context) {
        ctx.close(None);
    }
}

#[get("/api")]
async fn handle(
    req: HttpRequest,
    stream: Payload,
    app_data: web::Data<Mutex<State>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        Ws {
            app_data: app_data.clone(),
        },
        &req,
        stream,
    )
}
