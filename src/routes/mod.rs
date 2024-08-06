//Appel du controller hello_world
mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variable;
mod query_params;
mod mirror_user_agent;
mod mirror_custom_header;
mod middleware_message;
mod read_middleware_custom_header;
mod set_middleware_custom_header;

use axum::{
    body::Body, http::{method, Method}, middleware, routing::{
         get,post
   }, Extension, Router
    
};

//Appel du controller hello_world et la fonction hello_world 
use hello_world::hello_world;
use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;
use path_variable::{path_variable,hard_coded_path};
use query_params::query_params;
use mirror_user_agent::mirror_user_agent;
use mirror_custom_header::mirror_custom_header;
use middleware_message::middleware_message;
use read_middleware_custom_header::read_middleware_custom_header;
use set_middleware_custom_header::set_middleware_custom_header;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct SharedData{
    pub message:String,
}

pub fn create_routes() -> Router {
     
     let cors=CorsLayer::new()
     .allow_methods([Method::GET, Method::POST])
     .allow_origin(Any);

    let shared_data=SharedData{
        message:"Hello from shared data".to_owned()
    };

    Router::new()
    .route("/read_middleware_custom_header", get(read_middleware_custom_header))
    .route_layer(middleware::from_fn(set_middleware_custom_header))
    .route("/hello", get(hello_world))
    .route("/mirror_body_string",post(mirror_body_string))
    .route("/mirror_body_json",post(mirror_body_json))
    .route("/query_params",get(query_params))
    .route("/mirror_user_agent", get(mirror_user_agent))
    .route("/mirror_custom_header", get(mirror_custom_header))
    .route("/middleware_message", get(middleware_message))
    .route("/path_variable/15",get(hard_coded_path))
        .route("/path_variable/:id",get(path_variable))
    .layer(cors)
    .layer(Extension(shared_data))
   
    

}
