use poem::{get, handler, listener::TcpListener, post, web::{Json, Path}, Route, Server};
use crate::request_inputs::CreateWebsiteInput;
use crate::request_outputs::CreateWebsiteOutput;
use store::Store;

pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn get_website_status(Path(website_id): Path<String>) -> String {
    format!("GET status: {}", website_id)
}

//inspecting Json code shows it is pub struct Json<T>(pub T) -> which means it has exactly one field
//Json(data) is a way to destructure this struct to get the data directly, equivalent to doing data.0.url
#[handler]
fn create_website(Json(_data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let s = Store{};
    let id = s.create_website();
    let response = CreateWebsiteOutput{
        id: id
    };
    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    
    let app: Route = Route::new()
        .at("/website/:website_id", get(get_website_status))
        .at("/website", post(create_website));
        
    Server::new(TcpListener::bind("0.0.0.0:3001"))
        .run(app)
        .await
}