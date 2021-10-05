use mongodb::Collection;
use mongodb::bson::{doc, to_document};
use mongodb::bson::Bson::Null;
use mongodb::options::{FindOptions, FindOneAndUpdateOptions, ReturnDocument};
use futures::stream::TryStreamExt;
use std::sync::Arc;
use anyhow::{Context, anyhow};

use crate::comments_models::TodoComments;
use crate::error::AppError;

pub async fn insert_comment(
    collection: Arc<Collection<TodoComments>>, 
    todo_comments: TodoComments
) -> Result<TodoComments, AppError> {
    let todo_id = todo_comments.todo_id;
    let todo_comment = &todo_comments.comments[0];

    if let None = collection.find_one(doc! {"todo_id": todo_id}, None).await? {  // todo_idが該当するものが存在しなかった場合
        collection.insert_one(&todo_comments, None).await?;
        return Ok(todo_comments);
    } else {
        let fined_one_and_update_options =  FindOneAndUpdateOptions::builder()
            .return_document(Some(ReturnDocument::After))
            .build();
        let res = collection.find_one_and_update(
            doc!{"todo_id": todo_id},
            doc!{"$push":{"comments": to_document(todo_comment).context("bson decode error")?}},
            fined_one_and_update_options
            ).await?;

        match res {
            Some(res_todo_comments) => { return Ok(res_todo_comments); },
            None => { return Err(anyhow!("There is contradiction in logic").into()); }  //　ちゃんとインサートされていない場合
        }
    }
}

pub async fn upsert_comment(
    collection: Arc<Collection<TodoComments>>, 
    comment_id: i32, 
    todo_comments: TodoComments
) -> Result<TodoComments, AppError> {
    let todo_id = todo_comments.todo_id;
    let todo_comment = &todo_comments.comments[0];
    let comment = &todo_comments.comments[0].comment;

    if let None = collection.find_one(doc! {"todo_id": todo_id}, None).await? {  // todo_idが該当するものが存在しなかった場合
        collection.insert_one(&todo_comments, None).await?;
        return Ok(todo_comments);
    } else {
        let fined_one_and_update_options =  FindOneAndUpdateOptions::builder()
            .return_document(Some(ReturnDocument::After))
            .build();
        match collection.find_one_and_update(
            doc! {"todo_id": todo_id, format!("comments.{}",comment_id):{"$exists":true}},
            doc! { "$set":{format!("comments.{}.comment", comment_id): comment}},
            fined_one_and_update_options.clone()
        ).await? {
            Some(res_todo_comments) => {return Ok(res_todo_comments);},
            None => {
                let res_todo_comments = collection.find_one_and_update(
                    doc! {"todo_id": todo_id},
                    doc!{"$push":{"comments": to_document(todo_comment).context("bson decode error")?}},
                    fined_one_and_update_options.clone()
                ).await?
                .ok_or(anyhow!("There is contradiction in logic"))?;
                return Ok(res_todo_comments);
            }
        }
    }
}


pub async fn delete_comment(
    collection: Arc<Collection<TodoComments>>, 
    todo_id: i32,
    comment_id: i32
) -> Result<(), AppError> {
    if let None = collection.find_one(doc! {"todo_id": todo_id}, None).await? {  // todo_idが該当するものが存在しなかった場合
        return Err(AppError::CommentNotFoundError{todo_id, comment_id});
    } else {
        let res = collection.update_one(
            doc! {"todo_id": todo_id, format!("comments.{}", comment_id):{"$exists":true}},
            doc! {"$unset": {format!("comments.{}", comment_id): 1}},
            None
        ).await?;  // 該当コメントをnullにする
        if res.matched_count == 0 {  // todo_id, comment_idが該当するものが存在しなかった場合
            return Err(AppError::CommentNotFoundError{todo_id, comment_id});
        } else {
            collection.update_one(
                doc! {"todo_id": todo_id},
                doc!{"$pull": {"comments": Null}},
                None
            ).await?;  // nullのコメントを削除する

            collection.delete_one(
                doc! {"todo_id": todo_id, "comments": {"$size": 0}},
                None
            ).await?;  // comments配列の要素が全て削除された場合にドキュメントごと削除
        }

        return Ok(());
    }
}


pub async fn load_todo_comments(
    collection: Arc<Collection<TodoComments>>, 
    todo_ids: Vec<i32>
) -> Result<Vec<TodoComments>, AppError> {
    let find_options = FindOptions::builder().sort(doc! {"todo_id" : 1}).build();
    let cursor = collection.find(
        doc!{"todo_id":{"$in": todo_ids}},
        find_options
    ).await?;

    let todo_comments_vec: Vec<TodoComments> = cursor.try_collect().await?;
    Ok(todo_comments_vec)
}



