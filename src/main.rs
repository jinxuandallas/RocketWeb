mod models;
mod basic_auth;
mod schema;
//#[macro_use] extern crate diesel;
mod repositories;

use diesel::{RunQueryDsl, query_dsl::methods::{FilterDsl,SelectDsl, FindDsl, LimitDsl}, ExpressionMethods};

use bcrypt::{ verify, DEFAULT_COST};
// use diesel::associations::HasTable;
// use diesel::mysql::Mysql;
use rocket::{catch, catchers, delete, post, put, Config};
// use mysql::*;
// use mysql::prelude::Queryable;
use rocket::{get, http, launch, routes, State};
use rocket::figment::Figment;
// use rocket::figment::providers::{Format, Toml};
use rocket::serde::json::{json, Json, Value};
// use rocket::tokio::sync::Mutex;
use crate::models::models::{NewPerson, Person, User};
use rocket::fs::{relative, FileServer, NamedFile};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_sync_db_pools::{database};
use crate::basic_auth::BasicAuth;
// use crate::schema::person::dsl::*;
use crate::schema::{person, user};
use crate::schema::user::dsl::*;
// use crate::schema::user::{password, token, username};
// use diesel::prelude::*;
// use crate::schema::person;
use crate::repositories::PersonRepository;

#[database("mysql_path")]
struct DbConn(diesel::MysqlConnection);

// struct Dbconn{
//     conn:PooledConn
// }
// struct UserResult {
//     password:String,
//     token:String,
// }

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
async fn get_all(conn: DbConn,_auth:BasicAuth)->Value{
    // println!("{}",auth.0);
    // let result=LimitDsl::limit(person, 100).load::<Person>(&mut conn).await;
    conn.run(|con| {
        let persons=PersonRepository::find_all(con);
        json!(persons)
    }).await
    //let result:Vec<Person>=person.load::<Person>(&mut conn).await?;
    // let result:Vec<Row>=conn.query("SELECT * FROM person").unwrap();

    // let mut persons:Vec<Person>=Vec::new();
    // for row in result {
    //     persons.push( Person{
    //         id: row.get(0).unwrap(),
    //         name:row.get(1).unwrap(),
    //         age: row.get(2).unwrap(),
    //     });
    //
    // }
    //  json!(result)
}


#[post("/create",format="json", data="<new_person>")]
async fn create(conn:DbConn,new_person:Json<NewPerson>,_auth:BasicAuth)->Value {
    let add_person=new_person.into_inner();
    conn.run(move |con| {
       let inserted_rows = PersonRepository::create(con,&add_person);
        json!(inserted_rows)
    }).await
    /*
    let mut conn=& mut sconn.lock().await.conn;
    let stmt = conn.prep("INSERT INTO person (name, age) VALUES (:name, :age)")
        .unwrap();
    conn.exec_drop(&stmt, params! {
     "name" => add_person.name,
     "age" => add_person.age,
}).unwrap();
    json!(conn.last_insert_id())
    */

}


#[delete("/del/<id>")]
async fn delete(id:i32,conn:DbConn,_auth:BasicAuth)->Value {
    conn.run(move |con| {
        let num_deleted=PersonRepository::delete(con,id);
        json!(num_deleted)
    }).await
    /*
    let mut conn=& mut sconn.lock().await.conn;
    println!("{}",id);
    let stmt = conn.prep("delete from person where id=:id")
        .unwrap();
    conn.exec_drop(&stmt, params! {
     "id" => id,

}).unwrap();
    json!(conn.last_insert_id())

     */
}

#[put("/update",format="json", data="<person_data>")]
async fn update(conn:DbConn,person_data:Json<Person>,_auth:BasicAuth)->Value {
    let update_person=person_data.into_inner();
    conn.run(move |con| {
        PersonRepository::save(con,&update_person);
        json!("Successed")
    }).await
    /*
    let mut conn=& mut sconn.lock().await.conn;
    let stmt = conn.prep("update person set name=:name, age=:age where id=:id")
        .unwrap();
    conn.exec_drop(&stmt, params! {
     "name" => update_person.name,
     "age" => update_person.age,
     "id" => update_person.id,
}).unwrap();
    json!("Successed")

     */
}


#[post("/login",format="json",data="<user_data>")]
async fn login(conn:DbConn,user_data:Json<User>)->Value{
    let login_user=user_data.into_inner();
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
    conn.run(|con| {
        let result=user::table
            .filter(username.eq(login_user.username))
            .select((password,token))
            .first::<(String,String)>(con);


        match result {
            Ok((pwd,tok))=>{
                if verify(login_user.password,&pwd).unwrap()
                {
                    json!(format!("token:{}", &tok))
                }
                else {
                    return json!("密码错误")
                }

            }
            Err(_)=>{json!("用户名错误")}
        }
        // json!(result.unwrap())
    }).await

    /* let mut conn=& mut sconn.lock().await.conn;
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
*/
    //println!("{} {} {}",login_user.username,login_user.password,hash(&login_user.password,DEFAULT_COST).unwrap());
    //json!("Ok")
}



#[catch(401)]
async fn unauthorized() -> Value {
    json!("unauthorized")
}

#[launch]
fn rocket() -> _ {

    // let url = "mysql://root:Jinxuan2013@localhost:3306/test";
    // let pool = Pool::new(url).unwrap(); // 获取连接池
    //
    // let mut conn = pool.get_conn().unwrap();// 获取链接
    // let db_conn = Mutex::new(Dbconn{
    //     conn
    // });

    let cors= make_cors().to_cors().unwrap();

    let figment = Figment::from(rocket::Config::figment())
        // .merge(Toml::file("Rocket.toml").nested())
        .merge(("tls.certs","./ssl/cert.pem"))
        .merge(("tls.key","./ssl/key.pem"))
        .merge(("address","127.0.0.1"))
        .merge(("port",8000));


    // let config=Config::from(
    //     Figment::from(rocket::Config::default())
    //         .merge(("tls.certs","./ssl/cert.pem"))
    //         .merge(("tls.key","./ssl/key.pem"))
    //         .merge(("address","127.0.0.1"))
    //         .merge(("port",8000))
    //         // .merge(figment)
    //         // .merge(Toml::file("Rocket.toml").nested())
    //
    // );


    rocket::custom(figment)
        // .configure(&config)
        .attach(cors)
        // .manage(db_conn)
        // register routes
        .mount("/", routes![get_all,index,login,update,create,delete])
        // .mount("/del", routes![delete])
        .mount("/", FileServer::from(relative!("html")))
        .register("/", catchers!(unauthorized))
        .attach(DbConn::fairing())
}