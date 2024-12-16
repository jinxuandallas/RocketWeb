use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Person{
    pub id:u32,
    pub name: String,
    pub age:u32,
}

#[derive(Deserialize,Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User{
    pub username:String,
    pub password:String,
}