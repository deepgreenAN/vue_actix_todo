use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::schema::todos;

#[derive(Queryable, Debug, Serialize)]
pub struct Todo {  // todoデータベースでクエリする構造体
    pub todo_id: i32,
    pub todo: String,
    pub user_name: Option<String>,
    pub register_datetime: NaiveDateTime,
    pub finish_datetime: Option<NaiveDateTime>,
    pub done: bool
}

#[derive(Insertable, Debug)]
#[table_name = "todos"]
pub struct InsertTodo {  // todoデータペースに挿入する構造体
    pub todo: String,
    pub user_name: Option<String>,
    pub register_datetime: NaiveDateTime
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct InsertTodoForm {  // jsonで取得する構造体
    pub todo: String,
    pub user_name: Option<String>
}

// formに時間を添えてInsertTodoを返す
pub fn convert_todo_insert_form(
    insert_todo_form: InsertTodoForm,
    register_datetime: NaiveDateTime
) -> InsertTodo {
    InsertTodo {todo: insert_todo_form.todo, user_name: insert_todo_form.user_name, register_datetime}
}


#[derive(AsChangeset, Debug)]
#[table_name = "todos"]
pub struct UpdateTodo {  // todoデータベースに挿入する構造体
    pub todo: Option<String>,
    pub finish_datetime: Option<Option<NaiveDateTime>>,
    pub done: Option<bool>
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct UpdateTodoForm {  // jsonで取得する構造体
    pub todo_id: i32,
    pub todo: Option<String>,
    pub done: Option<bool>
}

// formに時間を添えてUpdateTodoを返す
pub fn convert_todo_update_form(
    update_todo_from: UpdateTodoForm,
    finish_datetime: Option<Option<NaiveDateTime>>
    ) -> (i32, UpdateTodo){
    (update_todo_from.todo_id, UpdateTodo{todo:update_todo_from.todo, finish_datetime, done:update_todo_from.done})
}