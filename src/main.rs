use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    // 多线程的 web 服务器
    //  - 在 socket 上监听 TCP 连接
    //  - 解析少量的 HTTP 请求
    //  - 创建一个合适的 HTTP 响应
    //  - 使用线程池改进服务器的吞吐量
    //  - 上述例子不是最佳实践
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // 请求
    // Method Request-URI HTTP-Version CRLF
    // headers CRLF
    // message-body

    // 响应
    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body

    let contents = fs::read_to_string("hello.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes()).unwrap();
    // flush 方法会等待并阻止程序的运行，直到所有的字节都被写入到连接中
    stream.flush().unwrap();
}