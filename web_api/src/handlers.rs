use iron::headers::ContentType;
use iron::{status, AfterMiddleware, Handler, IronResult, Request, Response};
use rustc_serialize::json;
use std::io::Read;
use std::sync::{Arc, Mutex}; //共享数据

use crate::database::Database;
use crate::models::Post;
use router::Router;
use std::error::Error;
use uuid::Uuid;

macro_rules! try_handler {
     ($e:expr) => {
          //表达式
          match $e {
               Ok(x) => x, //如果是表达式，返回表达式
               Err(e) => {
                    //否则返回error，并处理一下
                    return Ok(Response::with((
                         status::InternalServerError,
                         e.description(),
                    )));
               }
          }
     };
     ($e:expr,$error:expr) => {
          //两个表达式
          match $e {
               Ok(x) => x,
               Err(e) => return Ok(Response::with(($error, e.description()))),
          }
     };
}

//上锁
macro_rules! lock {
     ($e:expr) => {
          $e.lock().unwrap()
     };
}

//获取参数
macro_rules! get_http_param {
     ($r:expr,$e:expr) => {
          match $r.extensions.get::<Router>() {
               Some(router) => match router.find($e) {
                    Some(v) => v,
                    None => return Ok(Response::with(status::BadRequest)),
               },
               None => return Ok(Response::with(status::InternalServerError)),
          }
     };
}

pub struct Handlers {
     pub post_feed: PostFeedHandler,
     pub post_post: PostPostHandler,
     pub post: PostHandler,
}

impl Handlers {
     pub fn new(db: Database) -> Handlers {
          let database = Arc::new(Mutex::new(db));
          Handlers {
               post_feed: PostFeedHandler::new(database::clone()),
               post_post: PostPostHandler::new(database::clone()),
               post: PostHandler::new(database::clone()),
          }
     }
}

pub struct PostFeedHandler {
     //使用互斥器保护数据，使用Arc实现共享引用计数
     database: Arc<Mutex<Database>>,
}

impl PostFeedHandler {
     fn new(database: Arc<Mutex<Database>>) -> PostFeedHandler {
          PostFeedHandler { database }
     }
}
//为trait Handler 实现方法
impl Handler for PostFeedHandler {
     fn handle(&self, _: &mut Request) -> IronResult<Response> {
          let payload = try_handler!(json::encode(lock!(self.database).posts()));
          Ok(Response::with((status::Ok, payload)))
     }
}

pub struct PostPostHandler {
     database: Arc<Mutex<Database>>,
}

impl PostPostHandler {
     fn new(database: Arc<Mutex<Database>>) -> PostPostHandler {
          PostPostHandler { database }
     }
}

impl Handler for PostPostHandler {
     fn handle(&self, req: &mut Request) -> IronResult<Response> {
          let payload = String::new();
          try_handler!(req.body.read_to_string(&mut payload));
          let post = try_handler!(json::decode(&payload), status::BadRequest);

          lock!(self.database).add_post(post);
          Ok(Response::with((status::Created, payload)))
     }
}

pub struct PostHandler {
     database: Arc<Mutex<Database>>,
}

impl PostHandler {
     fn new(database: Arc<Mutex<Database>>) -> PostHandler {
          PostHandler { database }
     }

     fn find_post(&self, id: &Uuid) -> Option<Post> {
          let locked = lock!(self.database);
          let mut iterator = locked.posts().iter();
          iterator.find(|p| p.uuid() == *id).map(|p| p.clone())
     }
}

impl Handler for PostHandler {
     fn handle(&self, req: &mut Request) -> IronResult<Response> {
          let ref post_id = get_http_param!(req, "id");

          let id = try_handler!(Uuid::parse_str(post_id), status::BadRequest);
          if let Some(post) = self.find_post(&id) {
               let payload = try_handler!(json::encode(&post), status::InternalServerError);
               Ok(Response::with((status::Ok, payload)))
          } else {
               Ok(Response::with(status::NotFound))
          }
     }
}

pub struct JsonAfterMiddleWare;

impl AfterMiddleware for JsonAfterMiddleWare {
     fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
          res.headers.set(ContentType::json());
          Ok(res)
     }
}
