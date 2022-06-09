use actix_web::{Error, error, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::web::Data;
use sqlx::{Pool, Sqlite};

use crate::{AppState, Pasta};

pub async fn insert_pasta(data: &Data<AppState>, new_pasta: &Pasta)->Result<(), Error> {
    let mut conn = data.sqlite_pool.acquire().await.expect("sqlite conn error");

    sqlx::query(
        "
INSERT INTO pastas ( pasta_id, content, file, extension, private, editable, created, expiration, pasta_type)
VALUES ( ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ")
        .bind(&new_pasta.id)
        .bind(&new_pasta.content)
        .bind(&new_pasta.file)
        .bind(&new_pasta.extension)
        .bind(&new_pasta.private)
        .bind(&new_pasta.editable)
        .bind(&new_pasta.created)
        .bind(&new_pasta.expiration)
        .bind(&new_pasta.pasta_type)
        .execute(&mut conn)
        .await
        .map_err(|e|
            error::InternalError::new("Insert Error", StatusCode::INTERNAL_SERVER_ERROR)
        )?;
    Ok(())
}
