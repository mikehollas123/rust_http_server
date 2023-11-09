
use hollas_http:: {HttpServer};
fn main() {

    let mut server = HttpServer::new( "127.0.0.1",7879);
        server.add_route("GET / HTTP/1.1", doThing);

    server.run();
}

fn doThing<'a>() -> &'a str{
    println!("Did it!");

    "Did it!"

}
