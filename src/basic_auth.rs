use mysql::{params, Pool};
use mysql::prelude::Queryable;
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};

pub struct  BasicAuth(pub String);

impl BasicAuth {
    fn verify(token:&str) -> bool {
        let url = "mysql://root:Jinxuan2013@localhost:3306/test";
        let pool = Pool::new(url).unwrap(); // 获取连接池

        let mut conn = pool.get_conn().unwrap();

        let result:Option<i32> = conn.exec_first("select id from user where token=:token", params![token]).unwrap();

        match result {
            Some(_) => true,
            None => false
        }


    }
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let header_auth=request.headers().get_one("Authorization");

        if let Some(auth) = header_auth {
            let split_vec = auth.split_whitespace().collect::<Vec<&str>>();
            if split_vec.len() == 2 && split_vec[0] == "RocketWeb" && Self::verify(split_vec[1]) {
               return  Outcome::Success(BasicAuth(split_vec[1].to_string()))
            }
        }

             Outcome::Error((Status::Unauthorized, ()))

    }
}