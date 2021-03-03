use actix_web::{get, web, App, HttpResponse, HttpServer, ResponseError};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

struct TodooEntry{
    
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    let manager = SqliteConnectionManager::file("todo.db");
    let pool = Pool::new(manager).expect("Failed to initialize the connection pool.");
    let conn = pool
        .get()
        .expect("Failed to get the connection from the pool.");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL
        )",
        params![],
    )
    .expect("Failed to create a table 'todo'.");

    HttpServer::new(move || App::new().service(index).data(pool.clone()))
    .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}