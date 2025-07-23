use poem::{Route, Server, get, post, handler, listener::TcpListener, web::Path};


#[handler]
fn get_web_status(Path(status): Path<String>) -> String {
    format!("GET status: {}", status)
}

#[handler]
fn post_web_status(Path(status): Path<String>) -> String {
    format!("POST status: {}", status)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    
    let app = Route::new()
        .at("/status/:status", get(get_web_status).post(post_web_status));
        
    
    
    Server::new(TcpListener::bind("0.0.0.0:3001"))
        .run(app)
        .await
}