use crate::store::Store;
use diesel::prelude::*;

//Selectable brings codes for as_returning
#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]

struct User{
    username:String,
    password:String
}

impl Store{
    pub fn sign_up(&mut self, username: String, password: String) -> Result<String, diesel::result::Error>{
        let u = User {
            username,
            password
        };
        let result = diesel::insert_into(crate::schema::user)
        .values(&u)
        .returning(User::as_returning())
        .get_result(&mut self.conn);
    }

    pub fn sign_in(&self, username: String, password: String){

    }
}