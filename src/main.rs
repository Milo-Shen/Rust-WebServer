use std::net::TcpListener;

fn main() {
    // 多线程的 web 服务器
    //  - 在 socket 上监听 TCP 连接
    //  - 解析少量的 HTTP 请求
    //  - 创建一个合适的 HTTP 响应
    //  - 使用线程池改进服务器的吞吐量
    //  - 上述例子不是最佳实践
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();
    for stream in listener.incoming() {
        let steam = stream.unwrap();
        println!("Connection established");
    }
}
