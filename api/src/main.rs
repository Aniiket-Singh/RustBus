use poem::{get, handler, listener::TcpListener, post, web::{Json, Path}, Route, Server};
use uuid::Uuid; //for hard-coding uuid as input for calls for now
use crate::request_inputs::CreateWebsiteInput;
use crate::request_outputs::CreateWebsiteOutput;

// use crate::uuid::Uuid; 

use store::store::Store;

pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    format!("GET status: {}", website_id)
}

//inspecting Json code shows it is pub struct Json<T>(pub T) -> which means it has exactly one field
//Json(data) is a way to destructure this struct to get the data directly, equivalent to doing data.0.url
#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let mut s = Store::default().unwrap();
    //hardcoded user_id and url for testing
    let user_id = Uuid::new_v4().to_string();
    let website = s.create_website(user_id, data.url).unwrap();
    let response = CreateWebsiteOutput{
        id: website.id
    };
    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    
    let app: Route = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website));
        
    Server::new(TcpListener::bind("0.0.0.0:3001"))
        .run(app)
        .await
}