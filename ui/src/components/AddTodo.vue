<template>
  <div class="flex flex-col bg-green-400 w-120 h-32 border-2">
    <div class="flex flex-raw h-20">
      <div class="text-4xl"> ・ </div>
      <textarea class="resize-none w-100 h-20 text-2xl mt-2" v-model="todo"> </textarea>
    </div>
    <div class="flex flex-row h-12 mt-4">
      <div class="ml-2 w-24"> {{userName}} </div>
      <button class="rounded bg-red-400 active:bg-red-600 text-white text-sm mb-1"
        @click="addTodo()"
      > Todoの追加</button>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, SetupContext, ref} from 'vue';
import { AddTodoInterface } from './models';

type Props = {
  userName: string
}

export default defineComponent({
  name: "AddTodo",
  props:{
    userName: {
      type: String as () => string | null,
      required: false,
      default: null
    }
  },
  setup(_, context:SetupContext) {
    const todo = ref("");

    const addTodo = ():void => {
      if (todo.value !== "") {
        const event_todo_info: AddTodoInterface = {
          todo: todo.value
        };

        context.emit("add-todo", event_todo_info);
        todo.value = "";  // todoの初期化
      }
    }

    return {
      todo,
      addTodo
    }
  },
})
</script>