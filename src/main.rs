extern crate core;

use crate::args::ARGS;
use crate::endpoints::{
    create, edit, errors, help, pasta as pasta_endpoint, pastalist, remove, static_resources,
};
use crate::pasta::Pasta;
use actix_web::middleware::Condition;
use actix_web::{middleware, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::{env, fs};
use std::io::Write;
use std::sync::{Arc, RwLock};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;


pub mod args;
pub mod pasta;
mod repository;

pub mod util {
    pub mod animalnumbers;
    pub mod auth;
    pub mod misc;
    pub mod syntaxhighlighter;
}

pub mod endpoints {
    pub mod create;
    pub mod edit;
    pub mod errors;
    pub mod help;
    pub mod pasta;
    pub mod pastalist;
    pub mod remove;
    pub mod static_resources;
}

pub struct AppState {
    pub pg_pool: Arc<Pool<Postgres>>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    log::info!(
        "MicroBin starting on http://127.0.0.1:{}",
        ARGS.port.to_string()
    );

    let postgres_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            &env::var("DATABASE_URL").expect("DATABASE_URL not set")
        ).await.expect("pg problem");

    let data = web::Data::new(AppState {
        pg_pool: Arc::new(postgres_pool)
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(middleware::NormalizePath::trim())
            .service(create::index)
            .service(help::help)
            .service(pasta_endpoint::getpasta)
            .service(pasta_endpoint::getrawpasta)
            .service(pasta_endpoint::redirecturl)
            .service(edit::get_edit)
            .service(edit::post_edit)
            .service(static_resources::static_resources)
            .service(actix_files::Files::new("/file", "./pasta_data"))
            .service(web::resource("/upload").route(web::post().to(create::create)))
            .default_service(web::route().to(errors::not_found))
            .wrap(middleware::Logger::default())
            .service(remove::remove)
            .service(pastalist::list)
            .wrap(Condition::new(
                ARGS.auth_username.is_some(),
                HttpAuthentication::basic(util::auth::auth_validator),
            ))
    })
    .bind(format!("0.0.0.0:{}", ARGS.port.to_string()))?
    .workers(ARGS.threads as usize)
    .run()
    .await
}
