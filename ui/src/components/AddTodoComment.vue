<template>
  <div class="flex flex-col bg-green-400 h-32 w-48 border-2 p-2"> 
    <div class="flex flex-row h-12"> 
      <div class="h-7 w-20 text-sm"> {{userName}} </div>
      <input type=checkbox class="mt-1 ml-2" id=is-reply v-model="is_reply">
      <label for=is-reply>返信</label>
      <input type=number class="h-7 w-8 bg-white ml-1" min=0 :max="commentsLength-1" :disabled="!is_reply" v-model="reply_id_not_null">
    </div>
    <textarea class="resize-none h-20 w-full" v-model="comment"> </textarea>
    <div class="h-4">
      <button class="rounded bg-red-400 active:bg-red-600 text-white text-sm pl-1 pr-1" @click="addComment()">コメントの追加</button>
      <button class="rounded bg-red-400 active:bg-red-600 text-white text-sm ml-2 pl-1 pr-1" @click="clearContent()">クリア</button>
    </div>

  </div>
</template>

<script lang=ts>  
  import { defineComponent, SetupContext, ref, Ref} from "vue";
  import { AddTodoCommentInterface } from "./models";

  type Props = {
    commentsLength: number,
    userName: string | null
  }

  export default defineComponent({
    name: "AddTodoComment",
    props: {
      commentsLength: {
        type: Number,
        required: true,
      },
      userName: {
        type: String as () => string | null,
        required: false,
        default: null
      }
    },
    setup(props:Props, context:SetupContext) {
      const is_reply = ref(false);
      const reply_id_not_null = ref(0);  // nullでない場合のreply_id
      const comment = ref("");

      const addComment = ():void => {
        const reply_id: number | null = is_reply.value ?
          reply_id_not_null.value : null;

        if (comment.value!=="") {
          const event_comment_info: AddTodoCommentInterface = {
            user_name: props.userName,
            reply_id: reply_id,
            comment: comment.value
          };

          context.emit("add-comment", event_comment_info);
          comment.value = "";  // コメントの初期化
          
        }

      };

      const clearContent = ():void => {
        comment.value = "";
      };

      return {
        is_reply,
        reply_id_not_null,
        comment,
        addComment,
        clearContent
      }
      
    },
  })
</script>

  