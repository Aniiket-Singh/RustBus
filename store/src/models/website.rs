use crate::store::Store;
use diesel::prelude::*;
use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;

//Selectable brings codes for as_returning
#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::website)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct Website{
    pub id: String,
    pub url:String,
    pub user_id: String,
    pub created_at: NaiveDateTime
}


impl Store{
    pub fn create_website(&mut self, user_id: String, url: String) -> Result<Website, diesel::result::Error>{
        let id = Uuid::new_v4();
        let website = Website{
            user_id,
            id: id.to_string(),
            url,
            created_at: Utc::now().naive_utc()
        };

        let website = diesel::insert_into(crate::schema::website::table)
        .values(&website).returning(Website::as_returning())
        .get_result(&mut self.conn)?;

        Ok(website)
    }

    pub fn get_website(&mut self, website_id: String) -> Result<Website, diesel::result::Error>{
        use crate::schema::website::dsl::*;
        
        let website_result = website.filter(id.eq(website_id))
        .select(Website::as_select())
        .first(&mut self.conn)?;

        Ok(website_result)
    }
}