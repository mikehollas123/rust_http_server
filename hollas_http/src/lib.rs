use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}
};
use regex::Regex;

pub struct HttpServerBuilder<'a> {
    host : &'a str,
    port : i32,
    routes: Vec<&'a str>,
    functions: Vec<fn() ->&'a str>,
}

impl<'a> HttpServerBuilder<'a> {
    pub  fn new(addr : & str, port : i32) -> HttpServerBuilder {

        HttpServerBuilder{host: addr, port, routes: vec![], functions: vec![] }
    }

    pub fn add_route(&mut self, route : &'a str, function: fn() ->&'a str) -> &mut HttpServerBuilder<'a> {
        self.routes.push(route);
        self.functions.push(function);
        self
    }

    pub fn build(&mut self) -> HttpServer{
        HttpServer::new(self.host,self.port,&self.routes,&self.functions )
    }
}

pub struct HttpServer<'a> {
    host : &'a str,
    port : i32,
    routes: &'a Vec<&'a str>,
    functions: &'a Vec<fn() ->&'a str>,
}

impl<'a> HttpServer<'a> {
    fn new(addr : &'a str, port : i32, routes : &'a Vec<&'a str> , functions: &'a Vec<fn() ->&'a str>) -> HttpServer<'a> {

    HttpServer{host: addr, port, routes: &routes, functions: &functions }
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

    for (index,route) in self.routes.iter().enumerate()
    {
        let re = Regex::new(route).unwrap();

        if re.is_match(&http_request[0])
        {
            self.send_ok_content(self.functions[index](),stream);
            break;
        }
    }
}
    fn send_ok_content(&self,contents : &str,mut stream: TcpStream) {

        let status_line = "HTTP/1.1 200 OK";
        let length = contents.len();

        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}