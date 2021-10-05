#[macro_use]
extern crate diesel;

pub mod schema;
pub mod todos_models;
pub mod comments_models;
pub mod error;

pub mod todos_actions;
pub mod comments_actions;

pub mod change_counter;

use diesel::pg::PgConnection;

use diesel::select;
use diesel::RunQueryDsl;  // selectで必要
use diesel::r2d2::{self, ConnectionManager};

use mongodb::Collection;

use actix_web::{get, post, put, delete, web, Error, HttpResponse};
use actix_web::error::{
    ErrorInternalServerError,
    ErrorNotFound,
    ErrorBadRequest
};

use chrono::{NaiveDateTime, Utc};
use serde::Deserialize;
use anyhow::Context;
use tokio::time::{sleep, Duration};

use crate::todos_models::*;
use crate::comments_models::*;
use crate::todos_actions::*;
use crate::comments_actions::*;
use crate::error::AppError;
use crate::change_counter::ChangeCounter;


type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn count_change(change_counter: web::Data<ChangeCounter>) {  // 変更カウンタをインクリメント
    let mut counter = change_counter.counter.lock().unwrap(); 
    *counter = (*counter).wrapping_add(1_i32);
}

#[post("/api/todo")]
pub async fn insert_todo_hundler(
    pool: web::Data<DbPool>,
    counter: web::Data<ChangeCounter>,
    insert_todo_form: web::Json<InsertTodoForm>
) -> Result<HttpResponse, Error>{
    count_change(counter);
    let insert_todo_form: InsertTodoForm = insert_todo_form.0;
    let conn = pool.get().expect("couldn't get db connection from pool");
    
    web::block(move || { // 時間を計測してInsertTodoをinsert_todoへ
        let register_datetime: NaiveDateTime = select(diesel::dsl::now).get_result::<NaiveDateTime>(&conn).unwrap();
        let inserted_todo: InsertTodo = convert_todo_insert_form(insert_todo_form.clone(), register_datetime);
        insert_todo(&conn, inserted_todo)
    })
    .await?
    .map(|res_todo|HttpResponse::Ok().json(res_todo))
    .map_err(|some_err|ErrorInternalServerError(some_err))
}

#[put("/api/todo")]
pub async fn update_todo_hundler(
    pool: web::Data<DbPool>,
    counter: web::Data<ChangeCounter>,
    update_todo_form: web::Json<UpdateTodoForm>
) -> Result<HttpResponse, Error>{
    count_change(counter);
    let update_todo_form: UpdateTodoForm = update_todo_form.0;
    let conn = pool.get().expect("couldn't get db connection from pool");

    web::block(move || {
        let finish_datetime:Option<Option<NaiveDateTime>> = if let Some(true) = update_todo_form.done {
            select(diesel::dsl::now)
                .get_result::<chrono::NaiveDateTime>(&conn)
                .map(|timestamp|{Some(Some(timestamp))}).unwrap()
        } else {Some(None)}; // Noneを挿入するため

        let (todo_id, updated_todo): (i32, UpdateTodo) = convert_todo_update_form(update_todo_form.clone(), finish_datetime);
        update_todo(&conn, todo_id, updated_todo)
    })
    .await?
    .map(|res_todo|HttpResponse::Ok().json(res_todo))
    .map_err(|some_err|{
        match some_err {
            AppError::TodoNotFoundError{todo_id: _} => ErrorNotFound(some_err),
            _ => ErrorInternalServerError(some_err)
        }
    })
}

#[delete("/api/todo/{todo_id}")]
pub async fn delete_todo_hundler(
    pool: web::Data<DbPool>,
    counter: web::Data<ChangeCounter>,
    todo_id: web::Path<i32>
) -> Result<HttpResponse, Error>{
    count_change(counter);
    let conn = pool.get().expect("couldn't get db connection from pool");
    let todo_id = todo_id.into_inner();

    web::block(move || {
        delete_todo(&conn, todo_id)
    })
    .await?
    .map(|_|HttpResponse::Ok().finish())
    .map_err(|some_err|{
        match some_err {
            AppError::TodoNotFoundError{todo_id: _} => ErrorNotFound(some_err),
            _ => ErrorInternalServerError(some_err)
        }
    })
}

#[derive(Deserialize)]
pub struct TodoQuery{
    limit_num: i32,
    done: bool
}

#[get("/api/todo")]
pub async fn load_todo_hundler(pool: web::Data<DbPool>, query:web::Query<TodoQuery>) -> Result<HttpResponse, Error>{
    let limit_num = query.limit_num;
    let done = query.done;
    let conn = pool.get().expect("couldn't get db connection from pool");
    web::block(move || load_all_todo(&conn, done, limit_num))
    .await?
    .map(|todo_vec|HttpResponse::Ok().json(todo_vec))
    .map_err(|some_err|ErrorInternalServerError(some_err))
}

#[get("/api/todo/{user_name}")]
pub async fn load_todo_by_user_hundler(pool: web::Data<DbPool>, user_name: web::Path<String>, query:web::Query<TodoQuery>) -> Result<HttpResponse, Error> {
    let user_name: String = user_name.into_inner();
    
    let limit_num = query.limit_num;
    let done = query.done;

    let conn = pool.get().expect("couldn't get db connection from pool");
     web::block(move || load_user_todo(&conn, user_name.clone(), done, limit_num))
    .await?
    .map(|todo_vec|HttpResponse::Ok().json(todo_vec))
    .map_err(|some_err|ErrorInternalServerError(some_err))
}

#[post("/api/comment")]
pub async fn insert_comment_hundler(
    collection: web::Data<Collection<TodoComments>>,
    counter: web::Data<ChangeCounter>,
    inserted_comment_form: web::Json<UpsertTodoCommentForm>
) -> Result<HttpResponse, Error> {
    count_change(counter);
    let collection = collection.into_inner();
    let inserted_comment_form = inserted_comment_form.0;
    let finish_datetime = Utc::now().naive_utc();
    let (_, inserted_comment) = convert_comment_upsert_form(inserted_comment_form, finish_datetime);
    insert_comment(collection, inserted_comment)
    .await
    .map(|todo_comments|HttpResponse::Ok().json(todo_comments))
    .map_err(|some_err|ErrorInternalServerError(some_err))
}

#[put("/api/comment")]
pub async fn upsert_comment_hundler(
    collection: web::Data<Collection<TodoComments>>,
    counter: web::Data<ChangeCounter>,
    upserted_comment_form: web::Json<UpsertTodoCommentForm>
) -> Result<HttpResponse, Error> {
    count_change(counter);
    let collection = collection.into_inner();
    let upserted_comment_form = upserted_comment_form.0;
    let finish_datetime = Utc::now().naive_utc();
    let (comment_id, upserted_comment) = convert_comment_upsert_form(upserted_comment_form, finish_datetime);
    upsert_comment(collection, comment_id, upserted_comment)
    .await
    .map(|todo_comments|HttpResponse::Ok().json(todo_comments))
    .map_err(|some_err|ErrorInternalServerError(some_err))   
}

#[delete("/api/comment/{todo_id}/{comment_id}")]
pub async fn delete_comment_hundler(
    collection: web::Data<Collection<TodoComments>>,
    counter: web::Data<ChangeCounter>,
    path: web::Path<(i32, i32)>
) -> Result<HttpResponse, Error> {
    count_change(counter);
    let collection = collection.into_inner();
    let path = path.into_inner();
    let todo_id = path.0;
    let comment_id = path.1;
    delete_comment(collection, todo_id, comment_id)
    .await
    .map(|_|HttpResponse::Ok().finish())
    .map_err(|some_err|{
        match some_err {
            AppError::TodoNotFoundError{todo_id: _} => ErrorNotFound(some_err),
            _ => ErrorInternalServerError(some_err)
        }
    })
}

#[derive(Deserialize, Clone)]
pub struct TodoCommentsQeuery{
    todo_ids: String
}

pub fn parse_todo_ids_str(todo_ids_str: String) -> Result<Vec<i32>, AppError>{
    let todo_ids = todo_ids_str.split(",").map(|id_str|{
        id_str.parse::<i32>().context("url parse_error")
        .map_err(|some_err|some_err.into())
    }).collect::<Result<Vec<i32>, AppError>>()?;
    Ok(todo_ids)
}

#[get("/api/comment")]
pub async fn load_todo_comments_multi_hundler(
    collection: web::Data<Collection<TodoComments>>,
    query: web::Query<TodoCommentsQeuery>
) -> Result<HttpResponse, Error> {
    let collection = collection.into_inner();
    let todo_ids = parse_todo_ids_str(query.todo_ids.clone())
    .map_err(|some_err|ErrorBadRequest(some_err))?;

    load_todo_comments(collection, todo_ids)
    .await
    .map(|todo_comments_vec|HttpResponse::Ok().json(todo_comments_vec))
    .map_err(|some_err|ErrorInternalServerError(some_err))
}

#[get("/api/comment/{todo_id}")]
pub async fn load_todo_comments_one_hundler(
    collection: web::Data<Collection<TodoComments>>,
    todo_id: web::Path<i32>
) -> Result<HttpResponse, Error> {
    let collection = collection.into_inner();
    let todo_id = todo_id.into_inner();

    load_todo_comments(collection, vec![todo_id])
    .await
    .map(|todo_comments_vec|HttpResponse::Ok().json(todo_comments_vec))
    .map_err(|some_error|ErrorInternalServerError(some_error))
}    

#[get("/api/long_polling")]
pub async fn load_polling_hundler(
    counter: web::Data<ChangeCounter>
) -> Result<HttpResponse, Error>{
    let init_counter = *counter.counter.lock().unwrap();  // 最初に変更カウンタを取得
    let mut checked_counter = init_counter;
    while checked_counter == init_counter{
        sleep(Duration::from_secs(30)).await;
        checked_counter = *counter.counter.lock().unwrap();  // 非同期に待った後に変更カウンタを取得
    }
    Ok(HttpResponse::Ok().finish())
}

