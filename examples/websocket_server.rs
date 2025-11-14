/// WebSocket server example
/// 
/// Run with: cargo run --example websocket_server --features websocket

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Error};
use actix_web_actors::ws;
use rust_template::websocket::{WebSocketServer, WebSocketSession};

async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    _server: web::Data<WebSocketServer>,
) -> Result<HttpResponse, Error> {
    let session = WebSocketSession::new();
    ws::start(session, &req, stream)
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
<!DOCTYPE html>
<html>
<head>
    <title>WebSocket Test</title>
</head>
<body>
    <h1>WebSocket Test Client</h1>
    <div id="status">Disconnected</div>
    <div>
        <input type="text" id="message" placeholder="Enter message">
        <button onclick="sendMessage()">Send</button>
    </div>
    <div id="messages"></div>

    <script>
        const ws = new WebSocket('ws://localhost:8080/ws');
        const status = document.getElementById('status');
        const messages = document.getElementById('messages');

        ws.onopen = () => {
            status.textContent = 'Connected';
            status.style.color = 'green';
        };

        ws.onmessage = (event) => {
            const msg = document.createElement('div');
            msg.textContent = 'Received: ' + event.data;
            messages.appendChild(msg);
        };

        ws.onclose = () => {
            status.textContent = 'Disconnected';
            status.style.color = 'red';
        };

        function sendMessage() {
            const input = document.getElementById('message');
            const message = {
                type: 'MESSAGE',
                data: input.value
            };
            ws.send(JSON.stringify(message));
            input.value = '';
        }

        // Send ping every 5 seconds
        setInterval(() => {
            if (ws.readyState === WebSocket.OPEN) {
                ws.send(JSON.stringify({type: 'PING'}));
            }
        }, 5000);
    </script>
</body>
</html>
        "#,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = env_logger::try_init_from_env(env_logger::Env::new().default_filter_or("info"));

    let ws_server = WebSocketServer::new();

    println!("🚀 Starting WebSocket server on http://127.0.0.1:8080");
    println!("📝 Open http://127.0.0.1:8080 in your browser to test");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ws_server.clone()))
            .route("/", web::get().to(index))
            .route("/ws", web::get().to(ws_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

