use actix::{Actor, StreamHandler, Handler, Message as ActixMessage};
use actix_web_actors::ws;
use std::time::Instant;
use super::messages::{ClientMessage, ServerMessage};

/// WebSocket session
pub struct WebSocketSession {
    /// Client must send ping at least once per 10 seconds
    hb: Instant,
}

impl WebSocketSession {
    pub fn new() -> Self {
        Self { hb: Instant::now() }
    }

    fn handle_client_message(&mut self, msg: ClientMessage, ctx: &mut ws::WebsocketContext<Self>) {
        match msg {
            ClientMessage::Ping => {
                let response = ServerMessage::Pong;
                if let Ok(json) = serde_json::to_string(&response) {
                    ctx.text(json);
                }
            }
            ClientMessage::Subscribe { topic } => {
                tracing::info!("Client subscribed to topic: {}", topic);
                let response = ServerMessage::Subscribed { topic };
                if let Ok(json) = serde_json::to_string(&response) {
                    ctx.text(json);
                }
            }
            ClientMessage::Unsubscribe { topic } => {
                tracing::info!("Client unsubscribed from topic: {}", topic);
                let response = ServerMessage::Unsubscribed { topic };
                if let Ok(json) = serde_json::to_string(&response) {
                    ctx.text(json);
                }
            }
            ClientMessage::Message { topic, payload } => {
                tracing::info!("Received message on topic {}: {:?}", topic, payload);
                // Echo back for demo
                let response = ServerMessage::Message { topic, payload };
                if let Ok(json) = serde_json::to_string(&response) {
                    ctx.text(json);
                }
            }
        }
    }
}

impl Default for WebSocketSession {
    fn default() -> Self {
        Self::new()
    }
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        tracing::info!("WebSocket session started");
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        tracing::info!("WebSocket session stopped");
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                self.hb = Instant::now();
                if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                    self.handle_client_message(client_msg, ctx);
                } else {
                    let error = ServerMessage::Error {
                        message: "Invalid message format".to_string(),
                    };
                    if let Ok(json) = serde_json::to_string(&error) {
                        ctx.text(json);
                    }
                }
            }
            Ok(ws::Message::Binary(_)) => {
                tracing::warn!("Binary messages not supported");
            }
            Ok(ws::Message::Close(reason)) => {
                tracing::info!("WebSocket close: {:?}", reason);
                ctx.close(reason);
            }
            _ => {}
        }
    }
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct BroadcastMessage(pub ServerMessage);

impl Handler<BroadcastMessage> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, ctx: &mut Self::Context) {
        if let Ok(json) = serde_json::to_string(&msg.0) {
            ctx.text(json);
        }
    }
}

