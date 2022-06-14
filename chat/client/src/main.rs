use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000"; //服务器地址
const MSG_SIZE: usize = 32; //消息大小，可以自定义

fn main() {
    //惯例创建一个client
    let mut client = TcpStream::connect(LOCAL).expect("Steam failed to connected");

    //设置状态
    client
        .set_nonblocking(true)
        .expect("client failed to initialize");

    //设置发送者和接收者
    let (tx, rx) = mpsc::channel::<String>();

    //仍然是新建thread 处理每一个请求,注意，要循环啊
    thread::spawn(move || loop {
        //vec存储
        let mut buff = vec![0; MSG_SIZE];
        //匹配读的内容
        match client.read_exact(&mut buff) {
            //如果ok
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                println!("message recv {:?}", msg);
            }
            //如果出错
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("connection with server was served");
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("writing to socket failed");
                println!("message sent {:?}", msg);
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }
        thread::sleep(Duration::from_millis(100));
    });

    println!("write a message");
    //第二个loop
    loop {
        let mut buff = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("reading from stdin failed");
        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {
            break;
        }
    }
    println!("bye bye!")
}
