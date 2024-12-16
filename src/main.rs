mod models;


use rocket::{post, Config};
use mysql::*;
use mysql::prelude::Queryable;
use rocket::{get, http, launch, routes, State};
use rocket::figment::Figment;
use rocket::serde::json::{json, Json, Value};
use rocket::tokio::sync::Mutex;
use crate::models::models::{Person, User};
use rocket::fs::NamedFile;
use rocket_cors::{AllowedOrigins, CorsOptions};

struct Dbconn{
    conn:PooledConn
}

/// 解决跨域问题
fn make_cors() -> CorsOptions {
    let allowed_origins=AllowedOrigins::all();

    CorsOptions::default()
        .allowed_origins(allowed_origins)
        .allowed_methods(vec![http::Method::Get,http::Method::Post].into_iter().map(From::from).collect())
        .allowed_headers(rocket_cors::AllOrSome::All)
        .allow_credentials(true)
}
#[get("/")]
async fn index() -> NamedFile {
    //"hello".to_string()
    //et path=fs::canonicalize("/html/index.html").unwrap().to_str().unwrap().to_string();
    //path.to_string()
    //fs::canonicalize("./html/index.html").unwrap().display().to_string()
    NamedFile::open("./html/index.html").await.unwrap()
}
#[get("/getall")]
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


#[post("/login",format="json",data="<user>")]
async fn login(user:Json<User>)->Value{
    let login_user=user.into_inner();
    println!("{} {}",login_user.username,login_user.password);
    json!("Ok")
}


#[launch]
fn rocket() -> _ {

    let url = "mysql://root:Jinxuan2013@localhost:3306/test";
    let pool = Pool::new(url).unwrap(); // 获取连接池

    let mut conn = pool.get_conn().unwrap();// 获取链接
    let db_conn = Mutex::new(Dbconn{
        conn
    });

    let cors= make_cors().to_cors().unwrap();

    let config=Config::from(
        Figment::from(rocket::Config::default())
            .merge(("tls.certs","./ssl/cert.pem"))
            .merge(("tls.key","./ssl/key.pem"))
            .merge(("address","127.0.0.1"))
            .merge(("port",8000))

    );


    rocket::custom(config)
        .attach(cors)
        .manage(db_conn)
        // register routes
        .mount("/", routes![get_all,index,login])
}