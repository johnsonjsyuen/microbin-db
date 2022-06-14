
use actix_web::{get, web, HttpResponse};
use actix_web::http::{Error};
use askama::Template;

use crate::args::{Args, ARGS};
use crate::endpoints::errors::ErrorTemplate;
use crate::pasta::Pasta;
use crate::util::animalnumbers::to_u64;
use crate::AppState;
use crate::repository::read_pasta;

#[derive(Template)]
#[template(path = "pasta.html", escape = "none")]
struct PastaTemplate<'a> {
    pasta: &'a Pasta,
    args: &'a Args,
}

#[get("/pasta/{id}")]
pub async fn getpasta(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let id = to_u64(&*id.into_inner()).unwrap_or(0);

    println!("{}", id);

    match read_pasta(&data, &id).await {
        Some(Ok(found_pasta)) => {
            return Ok(HttpResponse::Ok().content_type("text/html").body(
                PastaTemplate {
                    pasta: &found_pasta,
                    args: &ARGS,
                }
                    .render()
                    .unwrap(),
            ))
        }
        Some(Err(_)) => {
            Ok(HttpResponse::InternalServerError().body("Query read Error"))
        }
        None => {
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(ErrorTemplate { args: &ARGS }.render().unwrap()))
        }
    }
}

#[get("/url/{id}")]
pub async fn redirecturl(data: web::Data<AppState>, id: web::Path<String>) -> HttpResponse {
    let id = to_u64(&*id.into_inner()).unwrap_or(0);

    match read_pasta(&data, &id).await {
        Some(Ok(found_pasta)) => {
            return if &found_pasta.pasta_type == "url" {
                HttpResponse::Found()
                    .append_header(("Location", String::from(&found_pasta.content)))
                    .finish()
            } else {
                HttpResponse::Ok()
                    .content_type("text/html")
                    .body(ErrorTemplate { args: &ARGS }.render().unwrap())
            }
        }
        Some(Err(_)) => {
            HttpResponse::InternalServerError().body("Query read Error")
        }
        None => {
            HttpResponse::Ok()
                .content_type("text/html")
                .body(ErrorTemplate { args: &ARGS }.render().unwrap())
        }
    }
}

#[get("/raw/{id}")]
pub async fn getrawpasta(data: web::Data<AppState>, id: web::Path<String>) -> String {
    let id = to_u64(&*id.into_inner()).unwrap_or(0);

    println!("{}", id);

    match read_pasta(&data, &id).await {
        Some(Ok(found_pasta)) => {
            found_pasta.content
        }
        Some(Err(_)) => {
            String::from("Query read Error")
        }
        None => {
            String::from("Pasta not found! :-(")
        }
    }


}
