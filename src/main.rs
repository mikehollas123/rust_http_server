use hollas_http:: {HttpServerBuilder};

fn main() {
    let _server = HttpServerBuilder::new( "127.0.0.1",7879)
        .add_route("GET /", do_thing)
        .add_route("GET /again", do_thing2)
        .build().run();
}

fn do_thing<'a>() -> &'a str{
    "Did it!"
}
fn do_thing2<'a>() -> &'a str{
    "Did it Again!"
}