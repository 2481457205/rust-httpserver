use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

//server 结构体
pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    //new方法传入ip地址返回self（server类型）
    pub fn new(socket_addr: &'a str) ->Self {
        Server {socket_addr}
    }
    //run方法监听http字节流运行server
    pub fn run(&self) {
        //通过TcpListener监听传进来的地址
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("running on {}", self.socket_addr);
        //使用incoming()方法 然后for循环进行处理
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("connetction established");
            //创建一个buffer用来读字节流 
            let mut read_buffer = [0;200];
            //把字节流读到buffer中
            stream.read(&mut read_buffer).unwrap();
            //把buffer转为vec然后unwrap()检测错误（请求结构体实现了From trait）可以用into()转为HttpResquest结构体
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            //用route处理请求决定调用的handler
            Router::route(req, &mut stream);

        }
    }
}