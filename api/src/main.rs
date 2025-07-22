use poem::{Route, Server, get, post, handler, listener::TcpListener, web::Path};


#[handler]
fn get_web_status(Path(status): Path<String>) -> String {
    format!("status: {}", status)
}

#[handler]
fn post_web_status(Path(status): Path<String>) -> String {
    format!("status: {}", status)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app: Route = Route::new()
        .at("/status/:status", get(get_web_status))
        .at("/status/:status",post(post_web_status));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}