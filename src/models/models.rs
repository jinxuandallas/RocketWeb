use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct Person{
    pub id:u32,
    pub name: String,
    pub age:u32,
}