use actix_web::{get, post, delete, web, App, HttpResponse, HttpServer};
// use serde::{Deserialize, Serialize};

#[actix_web::main]
async fn main() {
    match start_server_on(3000).await {
        Ok(_) => (),
        Err(err) => println!("{}", err)
    }
}

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

#[post("/")]
async fn keks() -> HttpResponse {
    HttpResponse::Ok().json("{\"test\":1}")
}

#[delete("/")]
async fn bla() -> HttpResponse {
    HttpResponse::Ok().body("Deleted")
}

async fn start_server_on(port: i32) -> Result<(), std::io::Error> {
    let server = HttpServer::new(|| 
        App::new().service(hello)
                  .service(keks)
                  .service(bla)
                  )
                  .bind(format!("127.0.0.1:{port}"))?;
    println!("Listening on port {port}");
    server.run().await
}  

// curl -d '{"rivers":[{"id":12,"length":12,"name":"Wasserhahn"}], "riverFlow":[{"river_id":12,"country_id":1}]}' -H "Content-Type: application/json" http://localhost:3000/
