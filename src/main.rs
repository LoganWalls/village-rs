use async_openai::{Client, config::OpenAIConfig, types::{CreateChatCompletionRequestArgs, ChatCompletionRequestMessageArgs, Role}};
use axum::{routing::get, Router, extract::State};
use maud::{html, Markup};
use serde::Deserialize;
use tower_http::services::ServeDir;

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};
use std::{time::Duration, sync::Arc};


pub struct AppState {
    pub llm_client: Client<OpenAIConfig>
}

#[tokio::main]
async fn main() {

    // set up LLM API
    let llm_client_config = OpenAIConfig::new().with_api_base("http://localhost:7777/v1");
    let llm_client = Client::with_config(llm_client_config);
    let app_state = Arc::new(AppState { llm_client });

    // build our application with a single route
    let app = Router::new()
        .nest_service("/", ServeDir::new("web-ui"))
        .route("/ws-chat", axum::routing::get(handler))
        .route("/clicked", get(clicked))
        .with_state(app_state);

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
pub async fn handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

#[derive(Deserialize)]
struct ChatMessage {
    message: String
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    while let Some(Ok(msg)) = socket.recv().await {
        let message = serde_json::from_slice::<ChatMessage>(&msg.into_data())
            .expect("websocket message to have correct format")
            .message;

        let llm_req = CreateChatCompletionRequestArgs::default().model("custom-model")
            .messages(vec![
                ChatCompletionRequestMessageArgs::default().role(Role::System).content("You are a helpful AI assistant named Zephyr.").build().unwrap(),
                ChatCompletionRequestMessageArgs::default().role(Role::User).content(&message).build().unwrap(),
            ])
            .max_tokens(200_u16)
            .build()
            .unwrap();
        
        let llm_resp = state.llm_client.chat().create(llm_req).await.unwrap();
        let llm_message = llm_resp.choices.first().unwrap().message.content.as_ref().unwrap();
        for c in llm_message.chars() {
            let html_response = html! { div id="history" hx-swap-oob="beforeend" { span { (c) } }};
            if socket
                .send(Message::Text(html_response.into()))
                .await
                .is_err()
            {
                // client disconnected
                return;
            }
            std::thread::sleep(Duration::from_millis(50))
        }
    }
}
