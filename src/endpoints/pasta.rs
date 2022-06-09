use actix_web::{get, web, HttpResponse, error};
use actix_web::http::{Error, StatusCode};
use askama::Template;

use crate::args::{Args, ARGS};
use crate::endpoints::errors::ErrorTemplate;
use crate::pasta::Pasta;
use crate::util::animalnumbers::to_u64;
use crate::util::misc::remove_expired;
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
    let mut pastas = data.pastas.write().unwrap();
    remove_expired(&mut pastas);

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
    let mut pastas = data.pastas.write().unwrap();

    let id = to_u64(&*id.into_inner()).unwrap_or(0);

    remove_expired(&mut pastas);

    for pasta in pastas.iter() {
        if pasta.id == id {
            if pasta.pasta_type == "url" {
                return HttpResponse::Found()
                    .append_header(("Location", String::from(&pasta.content)))
                    .finish();
            } else {
                return HttpResponse::Ok()
                    .content_type("text/html")
                    .body(ErrorTemplate { args: &ARGS }.render().unwrap());
            }
        }
    }

    HttpResponse::Ok()
        .content_type("text/html")
        .body(ErrorTemplate { args: &ARGS }.render().unwrap())
}

#[get("/raw/{id}")]
pub async fn getrawpasta(data: web::Data<AppState>, id: web::Path<String>) -> String {
    let mut pastas = data.pastas.write().unwrap();

    let id = to_u64(&*id.into_inner()).unwrap_or(0);

    remove_expired(&mut pastas);

    for pasta in pastas.iter() {
        if pasta.id == id {
            return pasta.content.to_owned();
        }
    }

    String::from("Pasta not found! :-(")
}
