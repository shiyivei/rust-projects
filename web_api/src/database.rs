use crate::models::Post;

//clone 是为了多线程处理
#[derive(Debug, Clone)]
pub struct Database {
     posts: Vec<Post>,
}

// 实现方法和关联函数
impl Database {
     //建一个空vector
     pub fn new() -> Database {
          Database { posts: vec![] }
     }

     //把post加入数据库
     pub fn add_post(&mut self, post: Post) {
          self.posts.push(post)
     }

     //把posts vec 以指针的形式返回
     pub fn posts(&self) -> &Vec<Post> {
          &self.posts
     }
}
