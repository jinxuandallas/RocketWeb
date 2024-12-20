use diesel::{Insertable, Queryable, Selectable};
use rocket::serde::{Deserialize, Serialize};
// use diesel::prelude::*;
// use crate::schema::person;

#[derive(Deserialize,Serialize,Queryable,Selectable,Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::person)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Person{
    pub ID:i32,//此处必须为i32，如果设置成u32或者其他的，会导致orm转换格式失败
    pub name: String,
    pub age:i32,//此处必须为i32，如果设置成u32或者其他的，会导致orm转换格式失败
}

#[derive(Insertable,Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::person)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewPerson{
    pub name: String,
    pub age:i32
}

#[derive(Deserialize,Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User{
    pub username:String,
    pub password:String,
}