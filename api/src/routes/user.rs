use std::sync::{Arc, Mutex};

use poem::{ handler, web::{Data, Json}};
// use uuid::Uuid; //for hard-coding uuid as input for calls for now
use crate::{request_inputs::CreateUserInput, request_outputs::SigninOutput};
use crate::request_outputs::CreateUserOutput;
use store::store::Store;


#[handler]
pub fn sign_up(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<CreateUserOutput>{
    let mut locked_s = match s.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            // Optionally log the error
            eprintln!("Mutex poisoned: {:?}", poisoned);
            // Recover watchdog, e.g., get the inner value
            poisoned.into_inner()
        }
    };
    let id = locked_s.sign_up(data.username, data.password).unwrap();
    let response = CreateUserOutput {
        id: id
    };

    Json(response)
}

#[handler]
pub fn sign_in(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<SigninOutput
>{
    let mut locked_s = s.lock().unwrap();
    let _exists = locked_s.sign_in(data.username, data.password).unwrap();

    let response = SigninOutput {
        jwt: String::from("jwt")
    };

    Json(response)
}
