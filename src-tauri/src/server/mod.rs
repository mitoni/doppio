use actix::Addr;
use actix_cors::Cors;
use actix_files;
use actix_web::{middleware, web, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::{io::Result, sync::Mutex};
use tauri::AppHandle;

use self::handlers::api::Ws;

mod handlers;

#[derive(Debug)]
pub struct Client {
    addr: Addr<Ws>,
    id: String,
}

#[derive(Debug)]
pub struct State {
    clients: Vec<Client>,
}

#[allow(dead_code)]
pub struct TauriAppState {
    app: Mutex<AppHandle>,
}

#[actix_web::main]
pub async fn init(app: AppHandle) -> Result<()> {
    let tauri_app = web::Data::new(TauriAppState {
        app: Mutex::new(app),
    });

    let state = web::Data::new(Mutex::new(State { clients: vec![] }));

    let key_path = tauri_app
        .app
        .lock()
        .unwrap()
        .path_resolver()
        .resolve_resource("cert/key.pem")
        .expect("failed to resolve resource");

    let cert_path = tauri_app
        .app
        .lock()
        .unwrap()
        .path_resolver()
        .resolve_resource("cert/cert.pem")
        .expect("failed to resolve resource");

    let static_path = tauri_app
        .app
        .lock()
        .unwrap()
        .path_resolver()
        .resolve_resource("src/server/handlers/static/")
        .expect("failed to resolve resource");

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(&key_path, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(&cert_path).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            // .app_data(tauri_app.clone())
            .wrap(middleware::Logger::default())
            .wrap(Cors::permissive())
            .service(handlers::api::handle)
            .service(handlers::find::handle)
            .service(actix_files::Files::new("/", &static_path).index_file("index.html"))
    })
    .bind(("0.0.0.0", 80))
    .expect("Something went wrong binding to http server")
    .bind_openssl("0.0.0.0:443", builder)
    .expect("Something went wrong binding to https server")
    .run()
    .await
}

// pub struct TauriAppState {
//     app: Mutex<AppHandle>,
// }
//
// #[derive(Debug)]
// pub struct Pippo {
//     count: Mutex<Box<i32>>,
// }
//
// #[actix_web::main]
// pub async fn init(app: AppHandle) -> Result<()> {
//     let tauri_app = web::Data::new(TauriAppState {
//         app: Mutex::new(app),
//     });
