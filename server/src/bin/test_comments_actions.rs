use mongodb::Client;
use chrono::Utc;

extern crate todo_app;
use todo_app::comments_models::*;
use todo_app::comments_actions::*;
use std::sync::Arc;

use anyhow::Result;


#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let mongo_db_url = std::env::var("MONGODB_URL").expect("MONGODB_URL must be set");
    let client = Client::with_uri_str(&mongo_db_url).await.expect("Failed to connect client");
    let db = client.database("todo_app");
    let collection = Arc::new(db.collection::<TodoComments>("todo_comments"));

    // データを一つ挿入
    let upserted_comment_form = UpsertTodoCommentForm{
        todo_id: 0,
        comment_id: 0,
        comment: "お湯が冷たすぎる".to_string(),
        user_name: Some("二郎".to_string()),
        reply_id: Some(0)
    };

    let finish_datetime = Utc::now().naive_local();
    let (comment_id, upserted_comment) = convert_comment_upsert_form(upserted_comment_form, finish_datetime);
    upsert_comment(collection.clone(), comment_id, upserted_comment).await?;

    // データを取得
    let vec_todo_comments = load_todo_comments(collection.clone(), vec![0]).await?;
    println!("vector of todo_comments:{:?}", vec_todo_comments);

    Ok(())
}