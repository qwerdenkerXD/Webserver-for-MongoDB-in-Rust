use actix_web::{get, post, delete, web, App, HttpResponse, HttpServer, web::Json};
use mongodb::{Client, Database, Collection, options::{ClientOptions, ResolverConfig}, bson::doc};
use serde::{Serialize, Deserialize};
use futures_util::TryStreamExt;  // necessary to get Vector from mongodb::Cursor (get all entries of a table)

#[actix_web::main]
async fn main() {
    // start webserver
    if let Err(err) = start_server_on(3000).await {
        println!("{}", err);
    }
}

async fn start_server_on(port: i32) -> Result<(), std::io::Error> {
    let server = HttpServer::new(|| 
        App::new().service(hello)

                  .service(get_city)
                  .service(get_cities)
                  .service(post_city)
                  .service(delete_city)

                  .service(get_country)
                  .service(get_countries)
                  .service(post_country)
                  .service(delete_country)

                  .service(get_river)
                  .service(get_rivers)
                  .service(post_river)
                  .service(delete_river)
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

// define schemas of mongo (without internal _id)
    #[derive(Serialize, Deserialize, Clone)]
    struct City {
        id: f32,
        name: String,
        coordinates: String,
        population: f32,
        country: f32
    }


    #[derive(Serialize, Deserialize, Clone)]
    struct Country {
        id: f32,
        name: String,
        is_democratic: bool,
        population: f32,
        capital: f32,
        #[serde(default)] /// not every country has the rivers-field in database -> added with empty Vector
        rivers: Vec<f32>
    }

    #[derive(Serialize, Deserialize, Clone)]
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

    #[get("/city/{id}")] // singular because /cities/{id} matches on /cities
    async fn get_city(path: web::Path<u32>) -> HttpResponse {
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
            return HttpResponse::InternalServerError().body("Internal Server Error: Incorrect data types or missing fields in serialization");
        }
    }

    #[get("/cities")]
    async fn get_cities() -> HttpResponse {
        // connect to mongo and get database
        let db = match connect_to_mongo().await{
            Ok(db) => db,
            Err(_) => {
                println!("Could not connect to mongodb");
                return HttpResponse::InternalServerError().body("Internal Server Error: Database not accessible")
            }
        };

        let cities: Collection<City> = db.collection("cities");
        if let Ok(city_cursor) = cities.find(None, None).await {
            let found_cities: Result<Vec<City>, _> = city_cursor.try_collect().await;
            match found_cities {
                Ok(f) => return HttpResponse::Ok().json(f),
                Err(_) => return HttpResponse::InternalServerError().body("Internal Server Error: Incorrect data types or missing fields in serialization"),
            }
        } else {
            return HttpResponse::InternalServerError().body("Internal Server Error: Secret room found");
        }
    }

    #[get("/country/{id}")]
    async fn get_country(path: web::Path<u32>) -> HttpResponse {
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
            return HttpResponse::InternalServerError().body("Internal Server Error: Incorrect data types or missing fields in serialization");
        }
    }

    #[get("/countries")]
    async fn get_countries() -> HttpResponse {
        // connect to mongo and get database
        let db = match connect_to_mongo().await{
            Ok(db) => db,
            Err(_) => {
                println!("Could not connect to mongodb");
                return HttpResponse::InternalServerError().body("Internal Server Error: Database not accessible")
            }
        };

        let countries: Collection<Country> = db.collection("countries");
        if let Ok(country_cursor) = countries.find(None, None).await {
            let found_countries: Result<Vec<Country>, _> = country_cursor.try_collect().await;
            match found_countries {
                Ok(f) => return HttpResponse::Ok().json(f),
                Err(err) => {println!("{}", err); return HttpResponse::InternalServerError().body("Internal Server Error: Incorrect data types or missing fields in serialization")},
            }
        } else {
            return HttpResponse::InternalServerError().body("Internal Server Error: Secret room found");
        }
    }

    #[get("/river/{id}")]
    async fn get_river(path: web::Path<u32>) -> HttpResponse {
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
            return HttpResponse::InternalServerError().body("Internal Server Error: Incorrect data types or missing fields in serialization");
        }
    }

    #[get("/rivers")]
    async fn get_rivers() -> HttpResponse {
        // connect to mongo and get database
        let db = match connect_to_mongo().await{
            Ok(db) => db,
            Err(_) => {
                println!("Could not connect to mongodb");
                return HttpResponse::InternalServerError().body("Internal Server Error: Database not accessible")
            }
        };

        let rivers: Collection<River> = db.collection("rivers");
        if let Ok(river_cursor) = rivers.find(None, None).await {
            let found_rivers: Result<Vec<River>, _> = river_cursor.try_collect().await;
            match found_rivers {
                Ok(f) => return HttpResponse::Ok().json(f),
                Err(_) => return HttpResponse::InternalServerError().body("Internal Server Error: Incorrect data types or missing fields in serialization"),
            }
        } else {
            return HttpResponse::InternalServerError().body("Internal Server Error: Secret room found");
        }
    }

// POST
    #[post("/city")]
    async fn post_city(post: Json<City>) -> HttpResponse {
        let city = post.into_inner();

        // connect to mongo and get database
        let db = match connect_to_mongo().await{
            Ok(db) => db,
            Err(_) => {
                println!("Could not connect to mongodb");
                return HttpResponse::InternalServerError().body("Internal Server Error: Database not accessible")
            }
        };

        let cities: Collection<City> = db.collection("cities");
        match cities.insert_one(city.to_owned(), None).await {
            Ok(_) => HttpResponse::Ok().json(city),
            Err(_) => return HttpResponse::InternalServerError().body("Internal Server Error: Insert failed")
        }
    }

    #[post("/country")]
    async fn post_country(post: Json<Country>) -> HttpResponse {
        let country = post.into_inner();

        // connect to mongo and get database
        let db = match connect_to_mongo().await{
            Ok(db) => db,
            Err(_) => {
                println!("Could not connect to mongodb");
                return HttpResponse::InternalServerError().body("Internal Server Error: Database not accessible")
            }
        };

        let countries: Collection<Country> = db.collection("countries");
        match countries.insert_one(country.to_owned(), None).await {
            Ok(_) => HttpResponse::Ok().json(country),
            Err(_) => return HttpResponse::InternalServerError().body("Internal Server Error: Insert failed")
        }
    }

    #[post("/river")]
    async fn post_river(post: Json<River>) -> HttpResponse {
        let river = post.into_inner();

        // connect to mongo and get database
        let db = match connect_to_mongo().await{
            Ok(db) => db,
            Err(_) => {
                println!("Could not connect to mongodb");
                return HttpResponse::InternalServerError().body("Internal Server Error: Database not accessible")
            }
        };

        let rivers: Collection<River> = db.collection("rivers");
        match rivers.insert_one(river.to_owned(), None).await {
            Ok(_) => HttpResponse::Ok().json(river),
            Err(_) => return HttpResponse::InternalServerError().body("Internal Server Error: Insert failed")
        }
    }

// DELETE
    #[delete("/city/{id}")]
    async fn delete_city(path: web::Path<u32>) -> HttpResponse {
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
        match cities.delete_many(doc!{ "id": city_id }, None).await {
            Ok(_) => HttpResponse::Ok().body(format!("Deleted city with id {city_id}")),
            Err(_) => return HttpResponse::InternalServerError().body("Internal Server Error: You achieved the impossible")
        }
    }

    #[delete("/country/{id}")]
    async fn delete_country(path: web::Path<u32>) -> HttpResponse {
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
        match countries.delete_many(doc!{ "id": country_id }, None).await {
            Ok(_) => HttpResponse::Ok().body(format!("Deleted country with id {country_id}")),
            Err(_) => return HttpResponse::InternalServerError().body("Internal Server Error: You achieved the impossible")
        }
    }

    #[delete("/river/{id}")]
    async fn delete_river(path: web::Path<u32>) -> HttpResponse {
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
        match rivers.delete_many(doc!{ "id": river_id }, None).await {
            Ok(_) => HttpResponse::Ok().body(format!("Deleted river with id {river_id}")),
            Err(_) => return HttpResponse::InternalServerError().body("Internal Server Error: You achieved the impossible")
        }
    }
