use actix_web::{Error, error, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::web::Data;
use sqlx::{Pool, Sqlite};

use crate::{AppState, Pasta};

pub async fn insert_pasta(data: &Data<AppState>, new_pasta: &Pasta) -> Result<(), Error> {
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

pub async fn read_pasta(data: &Data<AppState>, pasta_id: &i64) -> Option<Result<Pasta, Error>> {
    let mut conn = data.sqlite_pool.acquire().await.expect("sqlite conn error");

    let result = sqlx::query!(
        "
select * from pastas where pasta_id = ?
        ",
        pasta_id)
        .fetch_optional(&mut conn)
        .await
        .map_err(|e|
            error::InternalError::new("Query read Error", StatusCode::INTERNAL_SERVER_ERROR)
        );

    match result {
        Err(e) => Some(Err(e.into())),
        Ok(None) => None,
        Ok(Some(pasta)) => {
            let found_pasta = Pasta {
                id: pasta.pasta_id.unwrap(),
                content: pasta.content.unwrap(),
                file: pasta.file.unwrap(),
                extension: pasta.extension.unwrap(),
                private: pasta.private.unwrap(),
                editable: pasta.editable.unwrap(),
                created: pasta.created.unwrap(),
                expiration: pasta.expiration.unwrap(),
                pasta_type: pasta.pasta_type.unwrap(),
            };
            Some(Ok(found_pasta))
        }
    }
}

pub async fn list_pastas(data: &Data<AppState>) -> Result<Vec<Pasta>, Error> {
    let mut conn = data.sqlite_pool.acquire().await.expect("sqlite conn error");

    let result = sqlx::query!(
        "
select * from pastas
        ")
        .fetch_all(&mut conn)
        .await
        .map_err(|e|
            error::InternalError::new("Query read Error", StatusCode::INTERNAL_SERVER_ERROR)
        );

    match result {
        Err(e) => Err(e.into()),
        Ok(pastas) => {
            let listed_pastas: Vec<Pasta> = pastas.into_iter().map(|pasta|
                Pasta {
                    id: pasta.pasta_id.unwrap(),
                    content: pasta.content.unwrap(),
                    file: pasta.file.unwrap(),
                    extension: pasta.extension.unwrap(),
                    private: pasta.private.unwrap(),
                    editable: pasta.editable.unwrap(),
                    created: pasta.created.unwrap(),
                    expiration: pasta.expiration.unwrap(),
                    pasta_type: pasta.pasta_type.unwrap(),
                }).collect();
            Ok(listed_pastas)
        }
    }
}