use web_server;
use std::panic::catch_unwind;

fn main() {
    let port = 1234;
    connect(port)
}

fn connect(port: i32) {
    web_server::new()
        .get("/", Box::new(|request: web_server::Request, mut response: web_server::Response|{
            "Hello World!".into()
        }))
        .launch(port);
}
// curl -d '{"rivers":[{"id":12,"length":12,"name":"Wasserhahn"}], "riverFlow":[{"river_id":12,"country_id":1}]}' -H "Content-Type: application/json" http://localhost:3000/
