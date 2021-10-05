CREATE TABLE todos (
    todo_id SERIAL PRIMARY KEY,
    todo TEXT NOT NULL,
    user_name VARCHAR(10),
    register_datetime TIMESTAMP NOT NULL,
    finish_datetime TIMESTAMP,
    done BOOLEAN NOT NULL DEFAULT FALSE
)