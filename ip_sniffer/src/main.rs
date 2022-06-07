use std::env; //get param from env
use std::io::{self, Write}; //这些都是标准库中的类型和函数
use std::net::{IpAddr, TcpStream};
use std::process;
use std::str::FromStr; //convert str to ip_addr
use std::sync::mpsc::{channel, Sender};
use std::thread;

const MAX: u16 = 65535;

//用结构体表示输入的参数
struct Arguments {
    flag: String,
    threads: u16,
    ip_addr: IpAddr,
}

//为结构体实现方法
//对输入参数中的每个元素进行判断
impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        //判断参数个数
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }

        //解析第二个参数，第2个参数可以是flag（-h,-help,-j）也可以是ip(flag可以省略)，flag省略的情况下可以不填thread，但是有-j必须填thread
        let f = args[1].clone();
        if let Ok(ip_addr) = IpAddr::from_str(&f) {
            //if the f is ip
            return Ok(Arguments {
                flag: String::from(""), //默认空
                threads: 4,             //默认4线程
                ip_addr,
            });

            //如果解析失败，说明第二个参数一定是flag
        } else {
            let flag = args[1].clone(); //
                                        //如果是帮助flag，就打印
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!(
                    "Usage:-j to select how many threads you want 
                \r\n -h or -help to show this help message"
                );
                return Err("help");
                //如果是帮助flag，并且还有其他参数，打印错误
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("two many arguments");
                //判断是不是-j flag
                //program -j 10 192.168.1.1
            } else if flag.contains("-j") {
                if args.len() != 4 {
                    return Err("not enough arguments");
                }
                //解析第3个元素
                let threads = match args[2].parse::<u16>() {
                    //get num from string
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number"),
                };
                //解析第4个元素
                let ip_addr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid ip_addr; must be IPv4 or IPv6"),
                };

                //实例化结构体,三个成员都解析完毕
                return Ok(Arguments {
                    flag,
                    threads,
                    ip_addr,
                });
            } else {
                return Err("invalid syntax");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    //将收到的逐级增加的线程数+1赋值给绑定给端口
    let mut port = start_port + 1; //端口号等与线程数+1,1000个线程,端口号：1-1000
    loop {
        //用Tcp请求连接这些端口
        //使用match匹配连接情况，使用枚举结果匹配连接情况
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                // stdout函数返回一个Stdout,实现了flush方法（flush在write trait下）,flush返回Result,实现了unwrap方法
                io::stdout().flush().unwrap(); //刷新所有输出确保都能到达
                tx.send(port).unwrap() //发送端口，凡是返回枚举类型都能够用unwrap方法解决
            }
            Err(_) => {} //第二个匹配分支
        }
        if (MAX - port) <= num_threads {
            //不可以让那个端口号最大值溢出
            //假如线程数量（并行请求）是1000，u16 max = 65535,
            break;
        }
        port = port + num_threads; //当线程数为1000，port为1，1001,2001,3001,...,1002,2000,3000,....
    }
}

fn main() {
    let args: Vec<String> = env::args().collect(); //args can attain input

    // for i in &args {
    //     println!("{}", i)
    // }

    //get every element
    // let program = args[0].clone();
    // let flag = args[1].clone();
    // let threads = args[2].clone();
    // let ip_addr = args[3].clone();

    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} problems parsing arguments: {}", program, err);
            process::exit(0);
        }
    });

    let flag = arguments.flag;
    println!("flag is {}", flag);
    let num_threads = arguments.threads;
    let addr = arguments.ip_addr;
    let (tx, rx) = channel(); //发送者和接收者

    for i in 0..num_threads {
        let tx = tx.clone(); //这里克隆了一个新的tx，并将其扔进了线程
                             //并行往scan函数中传递数据
        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    let mut out = vec![]; //见一个空数组
                          //丢掉最先先创建的tx，说明了要关闭channel
    drop(tx);

    //遍历接受者拿到的值
    for p in rx {
        out.push(p)
    }
    println!(""); //cargo run -- -h
                  //排序           // ["target/debug/ip_sniffer", "-h"]
    out.sort();
    //遍历打印
    for v in out {
        println!("{} is open", v)
    }
}

//the command should like:__rust_force_expr!
// ip_sniffer.exe -h
// ip_sniffer.exe -j 10 192.168.1.1
// ip_sniffer.exe 192.168.1.1

// fn test_for(num: u16) {
//     for i in 0..num {
//         println!("{}", i)
//     }
// }

// #[cfg(test)]

// mod tests {

//     use super::*;

//     #[test]

//     fn it_should_works() {
//         let num = 5;
//         test_for(num);
//     }
// }
