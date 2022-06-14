use std::time::{SystemTime, UNIX_EPOCH};
use actix_web::{Error, error, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::web::Data;
use sqlx::{Pool};

use crate::{AppState, Pasta};

pub async fn insert_pasta(data: &Data<AppState>, new_pasta: Pasta) -> Result<(), Error> {
    let mut conn = data.pg_pool.acquire().await.expect("pg conn error");

    sqlx::query!(
        "
INSERT INTO public.pastas ( id, content, file, extension, private, editable, created, expiration, pasta_type)
VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9)
        ",
        new_pasta.id,
        new_pasta.content,
        new_pasta.file,
        new_pasta.extension,
        new_pasta.private,
        new_pasta.editable,
        new_pasta.created,
        new_pasta.expiration,
        new_pasta.pasta_type)
        .execute(&mut conn)
        .await
        .map_err(|e|
            error::InternalError::new("Insert Error", StatusCode::INTERNAL_SERVER_ERROR)
        )?;
    Ok(())
}

pub async fn read_pasta(data: &Data<AppState>, pasta_id: &i64) -> Option<Result<Pasta, Error>> {
    let mut conn = data.pg_pool.acquire().await.expect("sqlite conn error");

    let result = sqlx::query_as!(Pasta,
        "
select * from public.pastas where id = $1
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
            Some(Ok(pasta))
        }
    }
}

pub async fn list_pastas(data: &Data<AppState>) -> Result<Vec<Pasta>, Error> {
    let mut conn = data.pg_pool.acquire().await.expect("sqlite conn error");

    let timenow: i64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => {
            log::error!("SystemTime before UNIX EPOCH!");
            0
        }
    } as i64;

    // Overriding the macro type inference due to bug https://github.com/launchbadge/sqlx/issues/1294
    sqlx::query_as!(Pasta,
        r#"
select * from public.pastas where expiration > $1 or expiration = 0
        "#,
         timenow)
        .fetch_all(&mut conn)
        .await
        .map_err(|e|
            error::InternalError::new("Query read Error", StatusCode::INTERNAL_SERVER_ERROR).into()
        )
}