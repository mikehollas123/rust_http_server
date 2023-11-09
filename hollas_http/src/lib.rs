use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    collections::{HashMap}
};
use std::fmt::format;


pub struct HttpServer<'a> {
    host : &'a str,
    port : i32,
    routes :  HashMap<&'a str, fn()->&'a str>,
}

impl<'a> HttpServer<'a> {
    pub fn new(addr : & str, port : i32) -> HttpServer {

    HttpServer{host: addr, port, routes:  HashMap::new() }
    }

    pub fn add_route(&mut self, route : &'a str, function: fn() ->&'a str)  {
        self.routes.insert(route,function);
    }

    pub fn run(self) -> (){
        let listener = TcpListener::bind(format!("{}:{}",self.host,self.port)).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            self.handle_connection(stream);
        }
    }
fn handle_connection(&self,mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    //println!("Request: {:#?}", http_request);


    let thing = match   &self.routes.get(&http_request[0][..]) {
        Some(thing ) => thing,
        None => panic!(),
    };

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
}