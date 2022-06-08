use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "127.0.0.1:6000"; //服务器地址
const MSG_SIZE: usize = 32; //消息大小，可以自定义

//睡眠函数
fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}

fn main() {
    //创建一个server，绑定端口
    let server = TcpListener::bind(LOCAL).expect("listener failed to bind");
    //初始化
    server
        .set_nonblocking(true)
        .expect("failed to initialize non-blocking");

    //为即将到来的客户请求创建一个vec存储
    let mut clients = vec![];
    //创建发送者和接收者
    let (tx, rx) = mpsc::channel::<String>();

    //对所有收到的访问遍历
    loop {
        //如果请求正常收到（并解构），一个socket就是一个请求，也就是一个client
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr); //输出状态

            //克隆发送者在下文使用，克隆是为了获取所有权
            let tx = tx.clone();

            //try_clone 就是clone的意思，将clone获得的socket发送保存
            clients.push(socket.try_clone().expect("failed to clone client"));

            //创建一个线程（这个线程就是一个匿名函数=闭包，有的有参数，有的无参数，参数放在｜｜中）
            thread::spawn(move || loop {
                //闭包体

                //消息缓存
                let mut buff = vec![0; MSG_SIZE];

                //准确读取
                match socket.read_exact(&mut buff) {
                    //读取成功
                    Ok(_) => {
                        //将信息存放在collect中
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        //将信息转成字符串
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                        //打印信息
                        println!("{}: {:?}", addr, msg);
                        //将信息发送给通道的接收者
                        tx.send(msg).expect("Failed to send msg to rx");
                    }

                    // 读取失败，通过错误终中断连接
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("closing connection with:{}", addr);
                        break;
                    }
                }

                //沉睡
                sleep();
            });
        }
        //在通道另一端接受数据并恢复
        if let Ok(msg) = rx.try_recv() {
            //读取每个vec中的客户
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    //克隆信息变为bytes
                    let mut buff = msg.clone().into_bytes();
                    //重新设置长度
                    buff.resize(MSG_SIZE, 0);

                    //写入缓存
                    client.write_all(&buff).map(|_| client).ok()
                })
                // 最后再放入collect
                .collect::<Vec<_>>();
        }
        sleep();
    }
}
