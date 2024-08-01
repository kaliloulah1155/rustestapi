//Appel du controller hello_world
mod hello_world;
use axum::{
    Router, 
    routing::{
         get,post
   }
};

//Appel du controller hello_world et la fonction hello_world 
use hello_world::hello_world;

pub fn create_routes() -> Router {
    Router::new().route("/hello", get(hello_world))
}
