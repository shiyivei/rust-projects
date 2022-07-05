use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use uuid::Uuid;

//使用外部包中的变量构建我们自己的变量
//为结构体实现trait
#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Post {
     title: String,
     body: String,
     author: String,
     date_time: DateTime<UTC>,
     uuid: Uuid,
}
//为变量实现关联函数

impl Post {
     //常规操作的new函数
     pub fn new(
          title: &str,
          body: &str,
          author: &str,
          date_time: DateTime<UTC>,
          uuid: Uuid,
     ) -> Post {
          //不用通过中间变量，可以直接返回
          Post {
               //返回值需要写字段名,引用了字符串切片，所以要转为字符串
               title: title.to_string(),
               body: body.to_string(),
               author: author.to_string(),
               //变量可以不写字段名
               date_time,
               uuid,
          }
     }
     //返回一个字段的方法
     pub fn uuid(&self) -> Uuid {
          self.uuid
     }
}
