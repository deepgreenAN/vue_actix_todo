interface TodoCommentInterface {  // リアクティプのコメントデータ
  user_name: string | null,
  reply_id: number | null,
  timestamp: string,
  comment:string
}

interface AddTodoCommentInterface {  // 追加する際にイベントを通して親へ渡すコメントデータ
  user_name: string | null,
  reply_id: number | null,
  comment: string
}

interface UpsertTodoCommentFormInterface {  // apiでサーバーへ渡すコメントデータ
  todo_id: number,
  comment_id: number,
  comment: string,
  user_name: string | null,
  reply_id: number | null
}

interface TodoInterface {  // リアクティプのtodoデータ
  todo_id: number,
  todo: string,
  user_name: string | null,
  register_datetime: string
}

interface AddTodoInterface {  // イベントを通して親へ渡すコメントデータ
  todo: string
}

interface InsertTodoFormInterface {  // apiでpostする時に渡すtodoデータ
  todo: string,
  user_name: string | null
}

interface UpdateTodoFormInterface {
  todo_id: number,
  todo: string | null,
  done: boolean | null
}


interface TodoCommentsInterface {
  todo_id: number,
  comments: TodoCommentInterface[]
}


export {
  TodoCommentInterface,
  AddTodoCommentInterface,
  UpsertTodoCommentFormInterface, 
  TodoInterface, 
  AddTodoInterface,
  InsertTodoFormInterface,
  UpdateTodoFormInterface,
  TodoCommentsInterface
};