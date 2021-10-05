use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoComment {  // upsertで利用する構造体
    pub comment: String,
    pub user_name: Option<String>,
    pub reply_id: Option<i32>,
    pub timestamp: NaiveDateTime
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoComments {  // クエリ、upsertで利用する構造体
    pub todo_id: i32,
    pub comments: Vec<TodoComment>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpsertTodoCommentForm {  // jsonで渡される構造体
    pub todo_id: i32,
    pub comment_id: i32,
    pub comment: String,
    pub user_name: Option<String>,
    pub reply_id: Option<i32>
}


pub fn convert_comment_upsert_form(
    comment_form: UpsertTodoCommentForm, 
    timestamp: NaiveDateTime
) -> (i32, TodoComments) {
    let todo_comment = TodoComment{
        comment: comment_form.comment,
        user_name: comment_form.user_name,
        reply_id: comment_form.reply_id,
        timestamp
    };
    (comment_form.comment_id, TodoComments{todo_id: comment_form.todo_id, comments: vec![todo_comment]})
}



