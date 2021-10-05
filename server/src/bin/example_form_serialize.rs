
extern crate todo_app;
use todo_app::todos_models::*;
use todo_app::comments_models::* ;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let insert_todo_form1 = InsertTodoForm{
        todo: "顔を洗う".to_string(),
        user_name: Some("太郎".to_string()),
    };
    println!("insert_todo example:{}", serde_json::to_string(&insert_todo_form1)?);
    
    let insert_todo_form2 = InsertTodoForm{
        todo: "顔を洗う".to_string(),
        user_name: None
    };
    println!("insert_todo null example:{}", serde_json::to_string(&insert_todo_form2)?);

    let update_todo_form1 = UpdateTodoForm{
        todo_id: 0,
        todo: Some("起床する".to_string()),
        done: Some(true),
    };
    println!("update_todo example:{}", serde_json::to_string(&update_todo_form1)?);

    let update_todo_form2 = UpdateTodoForm{
        todo_id: 0,
        todo: None,
        done: None,
    };
    println!("update_todo null example:{}", serde_json::to_string(&update_todo_form2)?);    

    let upsert_comment_form1 = UpsertTodoCommentForm{
        todo_id: 1,
        comment_id: 0,
        comment: "石鹸を買いましたよ".to_string(),
        user_name: Some("太郎".to_string()),
        reply_id: Some(0),
    };
    println!("upsert_todo_comment example:{}", serde_json::to_string(&upsert_comment_form1)?);

    let upsert_comment_form2 = UpsertTodoCommentForm{
        todo_id: 1,
        comment_id: 0,
        comment: "石鹸を買いましたよ".to_string(),
        user_name: None,
        reply_id: None,
    };
    println!("upsert_todo_comment example:{}", serde_json::to_string(&upsert_comment_form2)?);
    
    Ok(())
}