use diesel::pg::PgConnection;
use diesel::prelude::*;

use chrono::NaiveDateTime;
use diesel::select;

extern crate todo_app;
use todo_app::todos_models::*;
use todo_app::todos_actions::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let sql_db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&sql_db_url)?;


    // データを挿入
    let insert_todo_form = InsertTodoForm{
        todo: "顔を洗う".to_string(),
        user_name: Some("太郎".to_string()),
    };
    let register_datetime: NaiveDateTime = select(diesel::dsl::now).get_result::<chrono::NaiveDateTime>(&conn).unwrap();
    let inserted_todo: InsertTodo = convert_todo_insert_form(insert_todo_form.clone(), register_datetime);
    insert_todo(&conn, inserted_todo)?;

    // データを取得(全部)して表示
    let limit_num = 5;
    let todo_vec =  load_all_todo(&conn, false, limit_num)?;
    println!("loaded todo vec:{:?}", todo_vec);
    
    // データを更新
    let update_todo_form = UpdateTodoForm{
        todo_id: 1,
        todo:Some("歯磨きをする".to_string()),
        done:Some(true)
    };
    let finish_datetime = Some(Some(register_datetime));
    let (todo_id, updated_todo): (i32, UpdateTodo) = convert_todo_update_form(update_todo_form, finish_datetime); 
    update_todo(&conn, todo_id, updated_todo)?;

    //　データを取得(特定のユーザーのみ)して表示
    let limit_num = 5;
    //let todo_vec = load_user_todo(&conn, "太郎".to_string(), limit_num)?;
    let todo_vec =  load_all_todo(&conn, false , limit_num)?;
    println!("loaded todo vec of 太郎:{:?}", todo_vec);
        
    Ok(())
}