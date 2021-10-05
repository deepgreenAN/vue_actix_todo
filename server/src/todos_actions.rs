use diesel::prelude::*;
use diesel::pg::PgConnection;

use diesel::RunQueryDsl;  // insert_intoで必要
use diesel::QueryDsl;  // ordered_byで必要
use diesel::insert_into;
use diesel::update;
use diesel::delete;
use diesel::result::Error;

use crate::todos_models::{InsertTodo, UpdateTodo, Todo};  // 各種構造体
use crate::error::AppError;

pub fn insert_todo(conn: &PgConnection, inserted_todo: InsertTodo) -> Result<Todo, AppError> {  // todoデータを挿入
    use crate::schema::todos::dsl::*;
    let res = insert_into(todos)
        .values(&inserted_todo)
        .get_result::<Todo>(conn)?;
    Ok(res)
}

pub fn update_todo(conn: &PgConnection, _todo_id: i32, updated_todo: UpdateTodo) -> Result<Todo, AppError> {  // todoデータを更新
    use crate::schema::todos::dsl::*;
    update(todos.find(_todo_id))
    .set(&updated_todo)
    .get_result::<Todo>(conn)
    .map_err(
        |some_err|{
            match some_err {
                Error::NotFound => AppError::TodoNotFoundError{todo_id:_todo_id},
                _ => AppError::SqlQueryError(some_err)
            }
        }
    )  
}

pub fn delete_todo(conn: &PgConnection, _todo_id: i32) -> Result<(), AppError> {  // todoデータを削除
    use crate::schema::todos::dsl::*;
    let deleted_row_n = delete(todos.find(_todo_id))
    .execute(conn)?;
    if deleted_row_n == 1 {  // 削除した行の数が1の場合
        Ok(())
    } else {
        Err(AppError::TodoNotFoundError{todo_id:_todo_id})
    }
}

pub fn load_all_todo(conn: &PgConnection, _done:bool, limit_num:i32) -> Result<Vec<Todo>, AppError> {  // todoデータを取得
    use crate::schema::todos::dsl::*;
    let todo_vec = todos
        .filter(done.eq(_done))
        .limit(limit_num as i64)
        .order_by(todo_id)
        .load::<Todo>(conn)?;
    Ok(todo_vec)
}

pub fn load_user_todo(conn: &PgConnection, _user_name: String, _done:bool, limit_num: i32) -> Result<Vec<Todo>, AppError> {  // 特定のユーザーのtodoデータを所得
    use crate::schema::todos::dsl::*;
    let todo_vec = todos
        .filter(user_name.eq(_user_name).and(done.eq(_done)))
        .limit(limit_num as i64)
        .order_by(todo_id)
        .load::<Todo>(conn)?;
    Ok(todo_vec)
}


