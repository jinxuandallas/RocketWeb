mod models;

use std::ptr::copy_nonoverlapping;
use mysql::*;
use mysql::prelude::Queryable;
use rocket::{get, launch, routes, State};
use rocket::serde::json::{json, Value};
use rocket::tokio::sync::Mutex;
use crate::models::models::Person;

struct Dbconn{
    conn:PooledConn
}


#[get("/")]
async fn get_all(sconn:&State<Mutex<Dbconn>>)->Value{
    let mut conn=& mut sconn.lock().await.conn;
    let result:Vec<Row>=conn.query("SELECT * FROM person").unwrap();

    let mut persons:Vec<Person>=Vec::new();
    for row in result {
        persons.push( Person{
            id: row.get(0).unwrap(),
            name:row.get(1).unwrap(),
            age: row.get(2).unwrap(),
        });

    }
    json!(persons)
}

#[launch]
fn rocket() -> _ {

    let url = "mysql://root:Jinxuan2013@localhost:3306/test";
    let pool = Pool::new(url).unwrap(); // 获取连接池

    let mut conn = pool.get_conn().unwrap();// 获取链接
    let db_conn = Mutex::new(Dbconn{
        conn
    });

    rocket::build()
        .manage(db_conn)
        // register routes
        .mount("/", routes![get_all])
}