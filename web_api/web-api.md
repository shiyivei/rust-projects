1.新建项目

2.引入依赖

3.构建模块并在main.rs中引入

4.导入外部包

```
extern crate chrono;
extern crate env_logger;
extern crate iron;
extern crate logger;
extern crate router;
extern crate rustc_serialize;
extern crate uuid;

mod models;
```

5.在mod文件中使用crate中的变量或者函数、方法

```
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
```

6.建一个数据库mod

```
use crate::models::Post;//引入同目录下的其他mod
```

