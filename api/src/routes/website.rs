use std::sync::{Arc, Mutex};

use poem::{handler, web::{Data, Json, Path}};
// use uuid::Uuid; //for hard-coding uuid as input for calls for now
use crate::{auth_middleware::UserId, request_inputs::CreateWebsiteInput, request_outputs::{CreateWebsiteOutput, GetWebsiteOutput}};
use store::store::Store;

#[handler]
pub fn get_website(
    Path(name): Path<String>, 
    Data(s): Data<&Arc<Mutex<Store>>>,
    UserId(user_id): UserId
    
    )-> Json<GetWebsiteOutput> {
    //could've loaded using user_id: UserId and used as user_id.0 too
    let mut locked_s = s.lock().unwrap();
    let website = locked_s.get_website(user_id,name).unwrap();
    Json(GetWebsiteOutput{
        url:website.url,
        id:website.id,
        user_id: website.user_id
    })
}

//inspecting Json code shows it is pub struct Json<T>(pub T) -> which means it has exactly one field
//Json(data) is a way to destructure this struct to get the data directly, equivalent to doing data.0.url
#[handler]
pub fn create_website(
    Json(data): Json<CreateWebsiteInput>, 
    Data(s): Data<&Arc<Mutex<Store>>>,
    UserId(user_id): UserId

) -> Json<CreateWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();
    //hardcoded user_id and url for testing
    // let id = Uuid::new_v4().to_string();
    // let user_id = id.to_string();
    let website = locked_s.create_website(user_id, data.url).unwrap();
    let response = CreateWebsiteOutput{
        id: website.id
    };
    Json(response)
}
