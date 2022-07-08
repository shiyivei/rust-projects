use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the number");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Please input a number");
    loop {
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("failed to read from stdin");

        let mut num: u32 = match num.trim().parse() {
            //解析要用match
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        }; //将输入的文字解析成数字
        match num.cmp(&secret_number) {
            Ordering::Equal => {
                println!("you win");
                break;
            }
            Ordering::Greater => println!("too big"),
            Ordering::Less => println!("too small"),
        }
    }
}
