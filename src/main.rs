use axum::{routing::get, Router};
use maud::{html, Markup};
use serde::Deserialize;
use tower_http::services::ServeDir;

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};
use std::time::Duration;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .nest_service("/", ServeDir::new("web-ui"))
        .route("/ws-chat", axum::routing::get(handler))
        .route("/clicked", get(clicked));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn clicked() -> Markup {
    html! {
       h1 { "Hi!" }
    }
}

// Below here is ws stuff
pub async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

#[derive(Deserialize)]
struct ChatMessage {
    message: String,
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        dbg!(&msg);
        let message = serde_json::from_slice::<ChatMessage>(&msg.into_data())
            .expect("websocket message to have correct format")
            .message;

        dbg!(&message);
        for c in message.chars() {
            let html_response = html! { div id="history" hx-swap-oob="beforeend" { span { (c) } }};
            if socket
                .send(Message::Text(html_response.into()))
                .await
                .is_err()
            {
                // client disconnected
                return;
            }
            std::thread::sleep(Duration::from_millis(100))
        }
    }
}
