use poem::{get, handler, listener::TcpListener, post, web::{Json, Path}, Route, Server};

use crate::request_inputs::CreateWebsite;

pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn get_website_status(Path(status): Path<String>) -> String {
    format!("GET status: {}", status)
}

#[handler]
fn create_website(Json(data): Json<CreateWebsite>) -> String {
    format!("POST status")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    
    let app = Route::new()
        .at("/status/:status", get(get_website_status))
        .at("/website", post(create_website));
        
    
    
    Server::new(TcpListener::bind("0.0.0.0:3001"))
        .run(app)
        .await
}