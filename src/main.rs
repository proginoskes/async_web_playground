//use std::thread::JoinHandle;


use axum::{
    routing::get,
    //Extension,
    Router,
};
use hyper::{Body, Request};
use tower_http::trace::TraceLayer;
use tower::ServiceBuilder;
use tracing::Span;
use chrono::prelude::Utc;

use tokio::signal;

fn log_request ( request : &Request<Body>, _span : &Span){
    println!("......");
    println!("{:} {:} || {:?}", 
            request.method(),
            request.uri(),
            Utc::now() 
        );
}

async fn server() {
    let service_log = ServiceBuilder::new()
        .layer(
            TraceLayer::new_for_http()
                .on_request(log_request)
        );
    // the app
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(service_log);

   

    // run it with hyper on localhost:3000
    match axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(axum_term())
        .await {
            Ok(_)=>{},
            Err(err)=>print!("{:}",err)
        }

}

async fn axum_term(){
    let ctrl_c = async{
        match signal::ctrl_c().await {
            Ok(_)=>{println!("shutting down")},
            Err(err)=>{println!("shutting down with error {:}", err)}
        }
    };

    tokio::select! {
        _=ctrl_c => {},
        _=std::future::pending::<()>() => {}
    }

    println!("goodbye!")
}

#[tokio::main]
async fn main() {
    //let token = CancellationToken::new();

    server().await

}
