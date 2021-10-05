use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use mongodb::Client;
use actix_web::{middleware, App, HttpServer};
use actix_web::web::Data;
use actix_files::Files;
use std::sync::Mutex;

extern crate todo_app;
use todo_app::*;
use todo_app::comments_models::TodoComments;
use todo_app::change_counter::ChangeCounter;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let sql_db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&sql_db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let mongo_db_url = std::env::var("MONGODB_URL").expect("MONGODB_URL must be set");
    let client = Client::with_uri_str(&mongo_db_url).await.expect("Failed to connect client");
    let db = client.database("todo_app");
    let collection = db.collection::<TodoComments>("todo_comments");

    let change_counter = Data::new(ChangeCounter{
        counter: Mutex::new(0_i32)
    });

    let bind = "127.0.0.1:8080";
    println!("starting server at: {}", &bind);

    let app_url = format!("http://{}/public", &bind);
    println!("app_url: {}", &app_url);

    let index_dir_path = std::env::var("INDEX_DIR_PATH").expect("INDEX_DIR_PATH must be set");

    HttpServer::new(move ||{
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(collection.clone()))
            .app_data(change_counter.clone())
            //.service(Files::new("/public", index_dir_path.clone()).index_file("index.html"))
            .service(Files::new("/public", index_dir_path.clone()).show_files_listing())
            .service(insert_todo_hundler)
            .service(update_todo_hundler)
            .service(delete_todo_hundler)
            .service(load_todo_hundler)
            .service(load_todo_by_user_hundler)
            .service(insert_comment_hundler)
            .service(upsert_comment_hundler)
            .service(delete_comment_hundler)
            .service(load_todo_comments_multi_hundler)
            .service(load_todo_comments_one_hundler)
            .service(load_polling_hundler)
    })
    .bind(&bind)?
    .run()
    .await
}