use std::error::Error;
use std::fs; //从文件系统中获取内容

pub struct Config {
     pub query: String,
     pub filename: String,
}

impl Config {
     pub fn new(args: &[String]) -> Result<Config, &'static str> {
          if args.len() < 3 {
               return Err("Not enough arguments");
          }
          let query = args[1].clone();
          let filename = args[2].clone();
          Ok(Config { query, filename })
     }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
     let contexts = fs::read_to_string(config.filename)?; //这个语句如果是解析正确，那就是String，失败返回err

     for line in search(&config.query, &contexts) {
          println!("{}", line)
     }
     Ok(()) //单元组（函数默认的返回值）
}

#[cfg(test)]

mod tests {
     use super::*;

     #[test]
     fn one_result() {
          let query = "duct";
          let contents = "\
Rust:
safe,fast,productive.
Pick three.";
          assert_eq!(vec!["safe,fast,productive."], search(query, contents))
     }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
     let mut results = Vec::new();
     for line in contents.lines() {
          if line.contains(query) {
               results.push(line)
          }
     }
     results
}
