use std::fmt::Debug;
use std::fmt::Display;

struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T); //定义类型里面的方法
}

impl<T, U> DoubleDrop<T> for U {
    //为trait实现trait
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;
    empty.double_drop(null); //这个操作会释放其本身和传入的参数类型

    let retangle = Retangle {
        length: 32,
        width: 46,
    };
    print_type(retangle)
}

fn print<T: Display>(t: T) {
    println!("{}", t)
}

fn print_type<T: Debug>(t: T) {
    //函数的泛型参数必须实现Debug
    println!("{:?}", t);
}

#[derive(Debug)] //通过宏为类型实现Debug trait
struct Retangle {
    length: u32,
    width: u32,
}
