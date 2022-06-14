use actix_multipart::Multipart;
use actix_web::{Error, get, HttpResponse, post, web};
use askama::Template;
use futures::TryStreamExt;

use crate::{AppState, ARGS, Pasta};
use crate::args::Args;
use crate::endpoints::errors::ErrorTemplate;
use crate::repository::{edit_pasta, read_pasta};
use crate::util::animalnumbers::to_u64;

#[derive(Template)]
#[template(path = "edit.html", escape = "none")]
struct EditTemplate<'a> {
    pasta: &'a Pasta,
    args: &'a Args,
}

#[get("/edit/{id}")]
pub async fn get_edit(data: web::Data<AppState>, id: web::Path<String>) -> HttpResponse {
    let id = to_u64(&*id.into_inner()).unwrap_or(0);

    match read_pasta(&data, &id).await {
        Some(Ok(found_pasta)) => {
            if found_pasta.editable {
                HttpResponse::Ok().content_type("text/html").body(
                    EditTemplate {
                        pasta: &found_pasta,
                        args: &ARGS,
                    }
                        .render()
                        .unwrap(),
                )
            } else {
                HttpResponse::Unauthorized().finish()
            }
        }
        Some(Err(_)) => {
            HttpResponse::InternalServerError().body("Query read Error")
        }
        None => {
            HttpResponse::NotFound().finish()
        }
    }
}

#[post("/edit/{id}")]
pub async fn post_edit(
    data: web::Data<AppState>,
    id: web::Path<String>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    if ARGS.readonly {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/"))
            .finish());
    }

    let id = to_u64(&*id.into_inner()).unwrap_or(0);

    let mut new_content = String::from("");

    while let Some(mut field) = payload.try_next().await? {
        match field.name() {
            "content" => {
                while let Some(chunk) = field.try_next().await? {
                    new_content = std::str::from_utf8(&chunk).unwrap().to_string();
                }
            }
            _ => {}
        }
    }

    match read_pasta(&data, &id).await {
        Some(Ok(found_pasta)) => {
            if found_pasta.editable {
                edit_pasta(&data, &found_pasta.id, &*new_content);

                return Ok(HttpResponse::Found()
                    .append_header(("Location", format!("/pasta/{}", &found_pasta.id_as_animals())))
                    .finish());
            } else {
                Ok(HttpResponse::Unauthorized().finish())
            }
        }
        Some(Err(_)) => {
            Ok(HttpResponse::InternalServerError().body("Query read Error"))
        }
        None => {
            Ok(HttpResponse::NotFound().finish())
        }
    }
}
