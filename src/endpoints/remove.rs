use actix_web::{get, web, HttpResponse};

use crate::args::ARGS;
use crate::endpoints::errors::ErrorTemplate;
use crate::util::animalnumbers::to_u64;
use crate::AppState;
use askama::Template;

#[get("/remove/{id}")]
pub async fn remove(_data: web::Data<AppState>, id: web::Path<String>) -> HttpResponse {
    if ARGS.readonly {
        return HttpResponse::Found()
            .append_header(("Location", "/"))
            .finish();
    }
    let _id = to_u64(&*id.into_inner()).unwrap_or(0);

    HttpResponse::Ok()
        .content_type("text/html")
        .body(ErrorTemplate { args: &ARGS }.render().unwrap())
}
