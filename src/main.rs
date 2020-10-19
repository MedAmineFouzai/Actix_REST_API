use std::io;
use actix_cors::Cors;
use actix_web::{App, Error, HttpResponse, HttpServer, Responder, get, middleware, web};

use futures::future::{err, Either};
use futures::{Future, Stream};


use listenfd::ListenFd;

mod service;


#[get("/")]
async fn classify() -> impl Responder {

    let result=service::classifications("5f8b4839d42642b5f38d78db").await;
    println!("{:?}",result);
    HttpResponse::Ok().body("ok")

}



#[actix_web::main]
async fn main()->io::Result<()>{

let mut listenfd=ListenFd::from_env();
let mut server=HttpServer::new(move ||{
            App::new()
                .wrap(middleware::Logger::default())
                .wrap(Cors::new()
                        .allowed_methods(vec!["GET"])
                        .supports_credentials()
                        .max_age(3600)
                        .finish(),
                    )
                .service(classify)
                });
server= if let Some(listener)=listenfd.take_tcp_listener(0).unwrap(){
            server.listen(listener)?
    }else{
            server.bind("127.0.0.1:8080")?
    };
server.run().await

}
