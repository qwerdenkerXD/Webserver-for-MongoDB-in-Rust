use actix_web::{get, post, delete, put, web, App, HttpResponse, HttpServer};
use mongodb::{Client, Database, Collection, options::{ClientOptions, ResolverConfig, FindOneOptions}, bson::{doc, Bson, from_bson, oid::ObjectId}};
use serde::{Serialize, Deserialize};

#[actix_web::main]
async fn main() {
    // start webserver
    if let Err(err) = start_server_on(3000).await {
        println!("{}", err);
    }
}

// define schemas of mongo (without internal _id)
#[derive(Serialize, Deserialize, Debug)]
struct City {
    id: f32,
    name: String,
    coordinates: String,
    population: f32,
    country: f32
}

#[derive(Serialize, Deserialize, Debug)]
struct Country {
    id: f32,
    name: String,
    is_democratic: bool,
    population: f32,
    capital: f32,
    rivers: Vec<f32>
}

#[derive(Serialize, Deserialize, Debug)]
struct River {
    id: f32,
    length: f32,
    name: String,
    countries: Vec<f32>
}

// GET
#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

#[get("/cities/{id}")]
async fn get_cities(path: web::Path<u32>) -> HttpResponse {
    let city_id = path.into_inner();

    // connect to mongo and get database
    let db = match connect_to_mongo().await{
        Ok(db) => db,
        Err(_) => {
            println!("Could not connect to mongodb");
            return HttpResponse::InternalServerError().body("Internal Server Error: Database not accessible")
        }
    };

    let cities: Collection<City> = db.collection("cities");
    if let Ok(city) = cities.find_one(doc! {"id": city_id}, None).await {
        match city {
            Some(c) => return HttpResponse::Ok().json(c),
            None => return HttpResponse::NotFound().body(format!("No city with id {city_id} found")),
        }
    } else {
        return HttpResponse::InternalServerError().body("Internal Server Error: Incorrect data types in serialization");
    }
}

#[get("/countries/{id}")]
async fn get_countries(path: web::Path<u32>) -> HttpResponse {
    let country_id = path.into_inner();

    // connect to mongo and get database
    let db = match connect_to_mongo().await{
        Ok(db) => db,
        Err(_) => {
            println!("Could not connect to mongodb");
            return HttpResponse::InternalServerError().body("Internal Server Error: Database not accessible")
        }
    };

    let countries: Collection<Country> = db.collection("countries");
    if let Ok(country) = countries.find_one(doc! {"id": country_id}, None).await {
        match country {
            Some(c) => return HttpResponse::Ok().json(c),
            None => return HttpResponse::NotFound().body(format!("No country with id {country_id} found")),
        }
    } else {
        return HttpResponse::InternalServerError().body("Internal Server Error: Incorrect data types in serialization");
    }
}

#[get("/rivers/{id}")]
async fn get_rivers(path: web::Path<u32>) -> HttpResponse {
    let river_id = path.into_inner();

    // connect to mongo and get database
    let db = match connect_to_mongo().await{
        Ok(db) => db,
        Err(_) => {
            println!("Could not connect to mongodb");
            return HttpResponse::InternalServerError().body("Internal Server Error: Database not accessible")
        }
    };

    let rivers: Collection<River> = db.collection("rivers");
    if let Ok(river) = rivers.find_one(doc! {"id": river_id}, None).await {
        match river {
            Some(c) => return HttpResponse::Ok().json(c),
            None => return HttpResponse::NotFound().body(format!("No river with id {river_id} found")),
        }
    } else {
        return HttpResponse::InternalServerError().body("Internal Server Error: Incorrect data types in serialization");
    }
}

// POST
#[post("/cities")]
async fn post_cities() -> HttpResponse {
    HttpResponse::Ok().json("{\"test\":1}")
}

// DELETE
#[delete("/cities")]
async fn delete_cities() -> HttpResponse {
    HttpResponse::Ok().body("Deleted")
}

// UPDATE
#[put("/cities")]
async fn put_cities() -> HttpResponse {
    HttpResponse::Ok().body("Deleted")
}

// update findOneAndReplace

async fn start_server_on(port: i32) -> Result<(), std::io::Error> {
    let server = HttpServer::new(|| 
        App::new().service(hello)
                  .service(get_cities)
                  .service(post_cities)
                  .service(delete_cities)
                  .service(get_countries)
                  // .service(post_countries)
                  // .service(delete_countries)
                  .service(get_rivers)
                  // .service(post_rivers)
                  // .service(delete_rivers)
                  )
                  .bind(format!("127.0.0.1:{port}"))?;
    println!("Listening on port {port}");
    server.run().await
}

async fn connect_to_mongo() -> Result<Database, Box<dyn std::error::Error>> {
    // An extra line of code to work around a DNS issue on Windows:
    let options =
      ClientOptions::parse_with_resolver_config("mongodb://localhost:27017", ResolverConfig::cloudflare())
         .await?;

    // create Client
    let client = Client::with_options(options)?;

    Ok(client.database("test"))
}

// curl -d '{"rivers":[{"id":12,"length":12,"name":"Wasserhahn"}], "riverFlow":[{"river_id":12,"country_id":1}]}' -H "Content-Type: application/json" http://localhost:3000/
