<template>
  <div class="flex flex-row w-264">
    <div class="flex flex-col bg-green-400 w-120 h-32 border-2">
      <div class="flex flex-raw h-20">
        <div class="text-xl w-6 mr-2 mt-2"> {{todoIndex}}. </div>
        <textarea class="resize-none w-100 h-20 text-2xl mt-2" 
          :value="todo"
          @input="$emit('update:todo', $event.target.value)"
        > </textarea>
        <div class="flex flex-col h-20 w-10 ml-2 mr-2">
          <button class="rounded bg-red-400 active:bg-red-600 text-white text-sm mt-2" @click="updateDone()"> 完了 </button>
          <button class="rounded bg-red-400 active:bg-red-600 text-white text-sm mt-2" @click="updateTodo()"> 更新 </button>
        </div>
      </div>
      <div class="flex flex-raw mt-4 h-12">
        <div class="ml-2 w-24"> {{todoInfo.user_name}} </div>
        <div class="text-white"> {{convert_datetime(todoInfo.register_datetime)}} </div>
      </div>
    </div>
    <div class="flex flex-row overflow-x-auto">
      <div v-for="(one_comment, index) in comments_list" :key="index">
        <todo-comment 
          :comment-info="one_comment"
          :comment-index="index"
          :reply-from="reply_from_list[index]" 
          v-model:comment="one_comment.comment"
          @update-comment="updateComment"
        ></todo-comment>
      </div>
    </div>
    <div>
      <add-todo-comment
        :user-name="appUserName"
        :comments-length="comments_list.length" 
        @add-comment="addComment"
      ></add-todo-comment>
    </div>
  </div>
</template>

<script lang=ts>
  import { defineComponent, PropType, SetupContext, reactive, onMounted} from 'vue';
  import TodoComment  from './TodoComment.vue';
  import AddTodoComment from './AddTodoComment.vue';
  import {
    TodoInterface, 
    TodoCommentInterface, 
    AddTodoCommentInterface, 
    UpsertTodoCommentFormInterface,
    TodoCommentsInterface
    } from './models';
  import {axios_base} from './axios_instances';
  import {AxiosError, AxiosResponse} from 'axios';
  import {convert_datetime} from './date';


  type Props = {
    todoInfo: TodoInterface,
    todo: string,
    appUserName: string | null
    todoIndex: number
  };
  
  export default defineComponent({
    name: "TodoWithComments",
    components: {
      TodoComment,
      AddTodoComment
    },
    props: {
      todoInfo: {
        type: Object as PropType<TodoInterface>,
        required: true,
      }, 
      todo:{
        type: String,
        required: true
      },
      appUserName:{
        type: String as () => string | null,
        required: false,
        default: null
      },
      todoIndex:{
        type: Number,
        required: true
      }
    },
    setup(props: Props, context: SetupContext) {
      // データの初期化
      const comments_list = reactive<TodoCommentInterface[]>([]);
      const reply_from_list = reactive<number[][]>([]);

      const loadComments = (all_todo_comments: TodoCommentInterface[]):void => {
        reply_from_list.splice(0);

        all_todo_comments.forEach((one_todo_comment:TodoCommentInterface, index:number) => {
          // 更新か追加(削除はない)
          if (index < comments_list.length) {
            comments_list[index] = {...one_todo_comment};
          } else {
            comments_list.push(one_todo_comment);
          }
          
          // reply_from_listに追加
          reply_from_list.push([]);
          if (one_todo_comment.reply_id != null) {
            reply_from_list[one_todo_comment.reply_id].push(index);
          }
        });
      };

      const addComment = (add_comment: AddTodoCommentInterface):void => {
        // 一時的にデータを追加
        const reactive_comment: TodoCommentInterface = {
          user_name: add_comment.user_name,
          reply_id: add_comment.reply_id,
          timestamp: "",
          comment: add_comment.comment
        };

        const reactive_comment_index = comments_list.length;
        comments_list.push(reactive_comment);
        reply_from_list.push([]);
        if (reactive_comment.reply_id != null){
          reply_from_list[reactive_comment.reply_id].push(reactive_comment_index);
        }

        // apiでデータをポスト
        const upsert_comment_form: UpsertTodoCommentFormInterface = {
          todo_id: props.todoInfo.todo_id,
          comment_id: reactive_comment_index,
          comment: reactive_comment.comment,
          user_name: reactive_comment.user_name,
          reply_id: reactive_comment.reply_id
        };

        axios_base.post<UpsertTodoCommentFormInterface, AxiosResponse<TodoCommentsInterface>>("/api/comment", upsert_comment_form)
        .then((res: AxiosResponse<TodoCommentsInterface>)=>{
          const posted_todo_comments: TodoCommentInterface[] = res.data.comments;
          loadComments(posted_todo_comments);
        })
        .catch((error: AxiosError<TodoCommentsInterface>)=>{
          console.log(error);
        });
          
      };

      const updateComment = async (comment_id: number):Promise<void> => {
        // 一時的にデータを更新
        const update_comment_form: UpsertTodoCommentFormInterface = {
          todo_id: props.todoInfo.todo_id,
          comment_id: comment_id,
          comment: comments_list[comment_id].comment,
          user_name: comments_list[comment_id].user_name,
          reply_id: comments_list[comment_id].reply_id
        };
        try {
          const todo_comments_res = await axios_base
          .put<UpsertTodoCommentFormInterface, AxiosResponse<TodoCommentsInterface>>("/api/comment", update_comment_form);

          const all_comments: TodoCommentInterface[] = todo_comments_res.data.comments;
          loadComments(all_comments);
        } catch(error){
          console.log(error);
        }
      };

      const updateTodo = ():void => {
        context.emit("update-todo", props.todoInfo.todo_id);
      };

      const updateDone = ():void => {
        context.emit("update-done", props.todoInfo.todo_id);
      };

      return {
        comments_list,
        reply_from_list,
        addComment,
        loadComments,
        updateComment,
        updateTodo,
        updateDone,
        convert_datetime
      }
    }

  })
</script>