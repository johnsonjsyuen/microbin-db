use actix_web::{get, web, HttpResponse, Error};
use askama::Template;

use crate::args::{Args, ARGS};
use crate::pasta::Pasta;
use crate::util::misc::remove_expired;
use crate::AppState;
use crate::repository::list_pastas;

#[derive(Template)]
#[template(path = "pastalist.html")]
struct PastaListTemplate<'a> {
    pastas: &'a Vec<Pasta>,
    args: &'a Args,
}

#[get("/pastalist")]
pub async fn list(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    if ARGS.no_listing {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/"))
            .finish());
    }

    let pastas = list_pastas(&data).await?;
    Ok(HttpResponse::Ok().content_type("text/html").body(
        PastaListTemplate {
            pastas: &pastas,
            args: &ARGS,
        }
            .render()
            .unwrap(),
    ))
}
