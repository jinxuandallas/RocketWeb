mod models;
mod basic_auth;

use bcrypt::{ verify, DEFAULT_COST};
use rocket::{catch, catchers, delete, post, put, Config};
use mysql::*;
use mysql::prelude::Queryable;
use rocket::{get, http, launch, routes, State};
use rocket::figment::Figment;
use rocket::serde::json::{json, Json, Value};
use rocket::tokio::sync::Mutex;
use crate::models::models::{Person, User};
use rocket::fs::{relative, FileServer, NamedFile};
use rocket_cors::{AllowedOrigins, CorsOptions};
use crate::basic_auth::BasicAuth;

struct Dbconn{
    conn:PooledConn
}
struct UserResult {
    password:String,
    token:String,
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
async fn get_all(sconn:&State<Mutex<Dbconn>>,_auth:BasicAuth)->Value{
    // println!("{}",auth.0);
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

#[post("/create",format="json", data="<person>")]
async fn create(sconn:&State<Mutex<Dbconn>>,person:Json<Person>,_auth:BasicAuth)->Value {
    let add_person=person.into_inner();
    let mut conn=& mut sconn.lock().await.conn;
    let stmt = conn.prep("INSERT INTO person (name, age) VALUES (:name, :age)")
        .unwrap();
    conn.exec_drop(&stmt, params! {
     "name" => add_person.name,
     "age" => add_person.age,
}).unwrap();
    json!(conn.last_insert_id())
}

#[delete("/del/<id>")]
async fn delete(id:i32,sconn:&State<Mutex<Dbconn>>,_auth:BasicAuth)->Value {
    let mut conn=& mut sconn.lock().await.conn;
    println!("{}",id);
    let stmt = conn.prep("delete from person where id=:id")
        .unwrap();
    conn.exec_drop(&stmt, params! {
     "id" => id,

}).unwrap();
    json!(conn.last_insert_id())
}

#[put("/update",format="json", data="<person>")]
async fn update(sconn:&State<Mutex<Dbconn>>,person:Json<Person>,_auth:BasicAuth)->Value {
    let update_person=person.into_inner();
    let mut conn=& mut sconn.lock().await.conn;
    let stmt = conn.prep("update person set name=:name, age=:age where id=:id")
        .unwrap();
    conn.exec_drop(&stmt, params! {
     "name" => update_person.name,
     "age" => update_person.age,
     "id" => update_person.id,
}).unwrap();
    json!("Successed")
}

#[post("/login",format="json",data="<user>")]
async fn login(sconn:&State<Mutex<Dbconn>>,user:Json<User>)->Value{
    let login_user=user.into_inner();
    //let pwd=hash(&login_user.password,DEFAULT_COST).unwrap();
    /*println!("{}",&pwd);

    let mut conn=& mut sconn.lock().await.conn;
    let result:Option<String>=conn.exec_first("SELECT token FROM user where username=:name and password=:pwd",params! {
        "name" => login_user.username.as_str(),
        "pwd"=> pwd,
    }).unwrap();

    match result {
        Some(token) => {json!(format!("token:{}", token))},
        None => {json!("failed")}
    }
    */
    let mut conn=& mut sconn.lock().await.conn;
    let result:Option<UserResult>=conn.exec_first("SELECT password,token FROM user where username=:name",params! {
        "name" => login_user.username.as_str(),
    }).map(
        |row|{
        row.map(|(pwd,tok)|UserResult{
            password:pwd,
            token:tok})
    }).
        unwrap();

    match result {
        Some(ur)=>{
            if verify(login_user.password,&ur.password).unwrap()
            {
                json!(format!("token:{}", &ur.token))
            }
            else {
                json!("密码错误")
            }

        }
        None=>{json!("用户名错误")}
    }

    //println!("{} {} {}",login_user.username,login_user.password,hash(&login_user.password,DEFAULT_COST).unwrap());
    //json!("Ok")
}

#[catch(401)]
async fn unauthorized() -> Value {
    json!("unauthorized")
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
        .mount("/", routes![get_all,index,login,update,create,delete])
        // .mount("/del", routes![delete])
        .mount("/", FileServer::from(relative!("html")))
        .register("/", catchers!(unauthorized))
}