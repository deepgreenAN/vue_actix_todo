use diesel;
use mongodb;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed blocking connection.")]
    ConnectionError(#[from] diesel::ConnectionError),
    
    #[error("sql query error")]
    SqlQueryError(#[from] diesel::result::Error),

    #[error("mongodb error")]
    MongoDBError(#[from] mongodb::error::Error),

    #[error("not found query 'todo_id = {todo_id:?}' for update")]
    TodoNotFoundError{todo_id: i32},

    #[error("not fond query 'todo_id = {todo_id:?}' 'comment_id = {comment_id:?}' for update")]
    CommentNotFoundError{
        todo_id: i32,
        comment_id: i32
    },

    #[error(transparent)]
    Other(#[from] anyhow::Error),

}
