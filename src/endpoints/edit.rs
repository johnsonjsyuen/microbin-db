use crate::args::Args;
use crate::dbio::save_to_file;
use crate::endpoints::errors::ErrorTemplate;
use crate::util::animalnumbers::to_u64;
use crate::{AppState, Pasta, ARGS};
use actix_multipart::Multipart;
use actix_web::{get, post, web, Error, HttpResponse};
use askama::Template;
use futures::TryStreamExt;

#[derive(Template)]
#[template(path = "edit.html", escape = "none")]
struct EditTemplate<'a> {
    pasta: &'a Pasta,
    args: &'a Args,
}

#[get("/edit/{id}")]
pub async fn get_edit(data: web::Data<AppState>, id: web::Path<String>) -> HttpResponse {
    let id = to_u64(&*id.into_inner()).unwrap_or(0);

    for pasta in pastas.iter() {
        if pasta.id == id {
            if !pasta.editable {
                return HttpResponse::Found()
                    .append_header(("Location", "/"))
                    .finish();
            }
            return HttpResponse::Ok().content_type("text/html").body(
                EditTemplate {
                    pasta: &pasta,
                    args: &ARGS,
                }
                .render()
                .unwrap(),
            );
        }
    }

    HttpResponse::Ok()
        .content_type("text/html")
        .body(ErrorTemplate { args: &ARGS }.render().unwrap())
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


            if pasta.editable {
                // TODO Implement repo method for edit
                pasta.content.replace_range(.., &*new_content);

                return Ok(HttpResponse::Found()
                    .append_header(("Location", format!("/pasta/{}", pastas[i].id_as_animals())))
                    .finish());
            }

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(ErrorTemplate { args: &ARGS }.render().unwrap()))
}
