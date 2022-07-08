use std::env; //从环境变量中获取参数
use std::process; //对进程进行处理

use minigrep::Config; //不应该报错,注意在main文件引入mod和引入lib.rs的区别
fn main() {
     let args: Vec<String> = env::args().collect(); //将输入值保存进变量

     let config = Config::new(&args).unwrap_or_else(|err| {
          println!("problem parsing argument: {}", err);
          process::exit(1)
     });

     if let Err(e) = minigrep::run(config) {
          eprintln!("Application error:{}", e);

          process::exit(1)
     };

     let v1 = vec![1, 2, 3, 4];
     let v1_iter = v1.iter(); //Iter([1, 2, 3, 4]) 把数组包起来了

     // for v in v1_iter {
     //      println!("{}", v)
     // }

     println!("{:?}", v1_iter)
}
