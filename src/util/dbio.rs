use std::fs::File;
use std::{env, io};
use std::io::{BufReader, BufWriter};
use futures::executor::block_on;
use sqlx::SqlitePool;

use crate::Pasta;

static DATABASE_PATH: &'static str = "pasta_data/database.json";

pub fn save_to_file(pasta_data: &Vec<Pasta>) {
    let mut file = File::create(DATABASE_PATH);
    match file {
        Ok(_) => {
            let writer = BufWriter::new(file.unwrap());
            block_on(save_to_db(pasta_data));
            serde_json::to_writer(writer, &pasta_data).expect("Failed to create JSON writer");
        }
        Err(_) => {
            log::info!("Database file {} not found!", DATABASE_PATH);
            file = File::create(DATABASE_PATH);
            match file {
                Ok(_) => {
                    log::info!("Database file {} created.", DATABASE_PATH);
                    save_to_file(pasta_data);
                }
                Err(err) => {
                    log::error!(
                        "Failed to create database file {}: {}!",
                        &DATABASE_PATH,
                        &err
                    );
                    panic!("Failed to create database file {}: {}!", DATABASE_PATH, err)
                }
            }
        }
    }
}

pub async fn save_to_db(pasta_data: &Vec<Pasta>)-> anyhow::Result<()> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    let mut conn = pool.acquire().await?;

    for pasta_org in pasta_data {
        let pasta = pasta_org.to_owned();
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
            .expect("inserting in db");
    };
        Ok(())
    }


pub fn load_from_file() -> io::Result<Vec<Pasta>> {
    let file = File::open(DATABASE_PATH);
    match file {
        Ok(_) => {
            let reader = BufReader::new(file.unwrap());
            let data: Vec<Pasta> = match serde_json::from_reader(reader) {
                Ok(t) => t,
                _ => Vec::new(),
            };
            Ok(data)
        }
        Err(_) => {
            log::info!("Database file {} not found!", DATABASE_PATH);
            save_to_file(&Vec::<Pasta>::new());

            log::info!("Database file {} created.", DATABASE_PATH);
            load_from_file()
        }
    }
}
