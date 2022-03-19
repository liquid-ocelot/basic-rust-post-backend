use rocket;

use crate::connection;
use crate::sample;
use rocket_cors::{ AllowedHeaders, AllowedOrigins};
use rocket::http::{ Method};

pub fn create_routes(){
    let allowed_origins = AllowedOrigins::All;

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().unwrap();


    rocket::ignite()
    .manage(connection::init_pool())
    .attach(cors)
    .mount("/post", 
        routes![
            sample::handler::all_posts,
            sample::handler::create_post,
            sample::handler::get_post,
            sample::handler::update_post,
            sample::handler::delete_post
        ],
    ).launch();
}