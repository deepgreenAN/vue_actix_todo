use chrono::Utc;
use std::sync::Arc;

use mongodb::bson::{doc, to_document};
use mongodb::Client;
use mongodb::options::FindOptions;

use futures::stream::TryStreamExt;
use anyhow::Result;

extern crate todo_app;  // 自身のパッケージをdbとして読み込む
use todo_app::comments_models::*;  // 各種構造体

#[actix_web::main]
//#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let mongo_db_url = std::env::var("MONGODB_URL").expect("MONGODB_URL must be set");
    let client = Client::with_uri_str(&mongo_db_url).await.expect("Failed to connect client");
    let db = client.database("todo_app");
    let collection = Arc::new(db.collection::<TodoComments>("todo_comments"));

    // 一つ挿入
    let now_time = Utc::now().naive_local();
    let todo_comment = TodoComments{
        todo_id: 3,
        comments: vec![
            TodoComment {
                comment: "服が乾いていなかった".to_string(),
                user_name: Some("太郎".to_string()),
                reply_id: None,
                timestamp: now_time,
            }
        ] 
    };

    if let None = collection.find_one(doc! {"todo_id": 3}, None).await? {  // todo_idが3のものが存在しなかった場合
        collection.insert_one(todo_comment, None).await?;
    }

    // コメントをupsert
    let new_comment1 = TodoComment {
        comment: "服のバリエーションが多い".to_string(),
        user_name: Some("太郎".to_string()),
        reply_id: None,
        timestamp: now_time,
    };

    let comment_id = 1;
    let todo_id = 3;

    let update_result = collection.update_one(
        doc!{"todo_id": todo_id, format!("comments.{}",comment_id):{"$exists":false}},
        doc!{"$push":{"comments": to_document(&new_comment1)?}},
        None
    ).await?;

    if update_result.matched_count < 1{  //上がマッチしない場合
        collection.update_one(
            doc! {"todo_id": todo_id},
            doc! { "$set":{format!("comments.{}.comment", comment_id): &new_comment1.comment}},
            None
        ).await?;
    }

    let new_comment2 = TodoComment {
        comment: "服が破けている".to_string(),
        user_name: Some("太郎".to_string()),
        reply_id: None,
        timestamp: now_time,
    };

    let update_result = collection.update_one(
        doc!{"todo_id": todo_id, format!("comments.{}",comment_id):{"$exists":false}},
        doc!{"$push":{"comments": to_document(&new_comment2)?}},
        None
    ).await?;
    if update_result.matched_count < 1{  //上がマッチしない場合
        collection.update_one(
            doc! {"todo_id": todo_id},
            doc! { "$set":{format!("comments.{}.comment", comment_id): &new_comment2.comment}},
            None
        ).await?;
    }

    // データを読み込んで表示
    println!("全文表示");
    let cursor = collection.find(None, None).await?;
    let todo_comments_vec: Vec<TodoComments> = cursor.try_collect().await?;
    println!("{:?}", todo_comments_vec);


    let todo_ids = vec![3];

    //部分的に読み込んで表示
    println!("特定のユーザーで検索");
    let find_options = FindOptions::builder().sort(doc! {"todo_id" : 1}).build();
    let cursor = collection.find(
        //doc!{"todo_id":{"$in": to_document(&todo_ids)?}},
        doc!{"todo_id":{"$in":  todo_ids}},
        find_options
    ).await?;

    let todo_comments_vec: Vec<TodoComments> = cursor.try_collect().await?;
    println!("{:?}", todo_comments_vec);

    Ok(())

}