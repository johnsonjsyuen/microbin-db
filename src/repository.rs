use actix_web::HttpResponse;
use sqlx::{Pool, Sqlite};

use crate::Pasta;


pub async fn insert_pasta(pool: &Pool<Sqlite>, pasta_ref: &Pasta) -> Result<(), HttpResponse> {
    let mut conn = pool.acquire().await.map_err(|e| {
        eprintln!("Unable to connect to Sqlite");
        HttpResponse::InternalServerError().finish()
    })?;

    let pasta = pasta_ref.to_owned();
    sqlx::query(
        "
INSERT INTO pastas ( pasta_id, content, file, extension, private, editable, created, expiration, pasta_type)
VALUES ( ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ")
        .bind(&pasta.id)
        .bind(&pasta.content)
        .bind(&pasta.file)
        .bind(&pasta.extension)
        .bind(&pasta.private)
        .bind(&pasta.editable)
        .bind(&pasta.created)
        .bind(&pasta.expiration)
        .bind(&pasta.pasta_type)
        .execute(&mut conn)
        .await
        .map_err(|e| {
            eprintln!("Failed to execute insert query");
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(())
}
