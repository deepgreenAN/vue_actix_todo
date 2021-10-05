table! {
    todos (todo_id) {
        todo_id -> Int4,
        todo -> Text,
        user_name -> Nullable<Varchar>,
        register_datetime -> Timestamp,
        finish_datetime -> Nullable<Timestamp>,
        done -> Bool,
    }
}
