<template>
  <app-header v-model:user-name="app_user_name"> </app-header>
  <div class="container mx-auto">
    <div v-for="(one_todo, index) in todo_list " :key="index">
      <todo-with-comments 
        :todo-info="one_todo"
        v-model:todo="one_todo.todo"
        :app-user-name="app_user_name"
        :todo-index="index"
        :ref="setChildref"
        @update-todo="updateTodo"
        @update-done="updateDone"
      ></todo-with-comments>
    </div>
    <add-todo :user-name="app_user_name" @add-todo="addTodo"></add-todo>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, Ref, reactive, onMounted} from 'vue';
import AppHeader from './components/AppHeader.vue';
import TodoWithComments from './components/TodoWithComments.vue';
import AddTodo from './components/AddTodo.vue';
import {
  TodoInterface, 
  AddTodoInterface, 
  InsertTodoFormInterface, 
  UpdateTodoFormInterface, 
  TodoCommentsInterface
  } from './components/models';
import {axios_base, axios_polling} from './components/axios_instances';
import {AxiosError, AxiosResponse} from 'axios';

export default defineComponent({
  name: 'App',
  components: {
    AppHeader,
    TodoWithComments,
    AddTodo
  },
  setup(){
    const app_user_name: Ref<string | null> = ref(null);  // ユーザー名
    // const use_todo_ids: number[] =  [] // 利用するtodo_idのリスト
    // const remove_todo_ids: number[] = [];  // 削除された(非表示化された)todo_idのリスト

    const todo_list = reactive<TodoInterface[]>([]);  // todoデータのリスト
    const todo_component_list: typeof TodoWithComments[] = [];
    const todo_id_index_map: Map<number, number> = new Map();

    const setChildref = (el: typeof TodoWithComments):void => {
      if (el) {
        todo_component_list.push(el);
      } 
    };

    const addTodo = (add_todo:  AddTodoInterface):void => {
      const reactive_todo: TodoInterface = {
        todo_id: todo_list.length,  // 一時的に渡す
        todo: add_todo.todo,
        user_name: app_user_name.value,
        register_datetime: ""
      };
      const reactive_todo_index = todo_list.length;
      todo_list.push(reactive_todo);

      // apiで通信
      const insert_todo_form: InsertTodoFormInterface = {
        todo: reactive_todo.todo,
        user_name: reactive_todo.user_name
      };

      axios_base.post<InsertTodoFormInterface, AxiosResponse<TodoInterface>>("/api/todo", insert_todo_form)
      .then((res: AxiosResponse<TodoInterface>):void => {
        const posted_todo: TodoInterface = res.data;
        todo_list[reactive_todo_index] = {...posted_todo};
        todo_id_index_map.set(reactive_todo.todo_id, reactive_todo_index);
      })
      .catch((error: AxiosError<TodoInterface>):void => {
        console.log(error);
      })

    }

    const makeCommentQuery = ():string => {
      return  Array.from(todo_id_index_map.keys()).map((todo_id:number)=>{return String(todo_id);}).join(",")
    }; 

    const loadTodoAndComments = async(done: boolean): Promise<void> =>{
      try {
        const todo_res = await axios_base.get<TodoInterface[]>("/api/todo", {
          params: {
            limit_num: 50,
            done: done
          }
        });

        // todoの更新
        const all_todo: TodoInterface[] = todo_res.data;  // 取得したtodo
        todo_component_list.splice(0);  // todo_component_listの要素を削除
        todo_id_index_map.clear();  // todo_id_index_mapの要素を削除
        all_todo.forEach((one_todo: TodoInterface, index:number) => {
          // todo_listの変更と追加
          if (index < todo_list.length){
            todo_list[index] = {...one_todo};
          } else {
            todo_list.push(one_todo);
          }
          todo_id_index_map.set(one_todo.todo_id, index);
        });

        //todoが存在する場合
        if (todo_list.length!=0) {
          const comment_query = makeCommentQuery();
          const comment_res = await axios_base.get<null, AxiosResponse<TodoCommentsInterface[]>>("/api/comment", {
            params: {
              todo_ids: comment_query
            }
          });

          // コメントの更新
          const all_todo_comments: TodoCommentsInterface[] = comment_res.data;  // 取得したコメント
          for (let one_todo_comments of all_todo_comments) {
            const index = todo_id_index_map.get(one_todo_comments.todo_id);  // コンポーネントのインデックスを取得
            if (typeof index != "undefined") {
              const child_component: typeof TodoWithComments = todo_component_list[index];
              child_component.loadComments(one_todo_comments.comments);
            }
          }
        }
      } catch(error) {
        console.log(error);
      }
    };

    const updateTodo = async (todo_id:number):Promise<void> => {
      const update_todo_index: number | undefined = todo_id_index_map.get(todo_id);
      if (typeof update_todo_index != "undefined") {
        const update_todo: string = todo_list[update_todo_index].todo;

        const update_todo_form: UpdateTodoFormInterface = {
          todo_id: todo_id,
          todo: update_todo,
          done: null
        };

        try {
          const todo_res = await axios_base.put<UpdateTodoFormInterface, AxiosResponse<TodoInterface>>
          ("/api/todo", update_todo_form);
          const updated_todo: TodoInterface = todo_res.data
          todo_list[update_todo_index] = {...updated_todo};
        } catch(error) {
          console.log(error);
        }
      }
    };

    const updateDone = async (todo_id:number):Promise<void> => {
      const update_todo_index: number | undefined = todo_id_index_map.get(todo_id);
      if (typeof update_todo_index != "undefined") {
        // todo_list, todo_id_index_mapから削除
        todo_list.splice(update_todo_index, 1);
        todo_id_index_map.delete(todo_id);

        const update_done_form: UpdateTodoFormInterface = {
          todo_id: todo_id,
          todo: null,
          done: true
        }

        try {
          await axios_base.put<UpdateTodoFormInterface, AxiosResponse<TodoInterface>>("/api/todo", update_done_form);
        } catch(error) {
          console.log(error);
        }
      }
    };

    const longPolling = async ():Promise<void> => {
      for(;;) {
        try {
          await axios_polling.get("/api/long_polling");
          await loadTodoAndComments(false);
        } catch(error) {
          console.log(error);
          break;
        }
      }
    }    

    const mount_func = onMounted(async ():Promise<void> => {
      await loadTodoAndComments(false);
      await longPolling();
    });


    return {
      app_user_name,
      todo_list,
      addTodo,
      setChildref,
      todo_component_list,
      todo_id_index_map,
      updateTodo,
      updateDone
    }
  }
});
</script>

<style>
* {
  margin: 0px;
  padding: 0px;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 0px;
}
</style>
