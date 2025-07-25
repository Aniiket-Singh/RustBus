use std::sync::{Arc, Mutex};

use poem::{ handler, http::{StatusCode}, web::{Data, Json}, Error};
// use uuid::Uuid; //for hard-coding uuid as input for calls for now
use crate::{request_inputs::CreateUserInput, request_outputs::SigninOutput};
use crate::request_outputs::CreateUserOutput;
use store::store::Store;


#[handler]
pub fn sign_up(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Result<Json<CreateUserOutput>, Error>{
    let mut locked_s = s.lock().unwrap();
    let id = locked_s.sign_up(data.username, data.password).map_err(|_| Error::from_status(StatusCode::CONFLICT))?; 

    let response = CreateUserOutput {
        id
    };
    Ok(Json(response))
}

#[handler]
pub fn sign_in(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Result<Json<SigninOutput>, Error>{
    let mut locked_s = s.lock().unwrap();
    let user_id = locked_s.sign_in(data.username, data.password);

    match user_id {
        Ok(user_id) => {
            let response = SigninOutput {
                jwt: user_id
            };
            Ok(Json(response))
        }
        Err(_ime) => {
            return Err(Error::from_status(StatusCode::UNAUTHORIZED));
        }
    }

}
