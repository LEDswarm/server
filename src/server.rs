use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use std::collections::HashMap;

use crate::controller::Controller;

pub struct WebsocketServer {
    pub controllers: HashMap<u8, Controller>,
}

impl Actor for WebsocketServer {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebsocketServer {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}
