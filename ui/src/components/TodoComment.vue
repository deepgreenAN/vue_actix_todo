<template>
  <div class="flex flex-col bg-green-400 h-32 w-48 border-2 p-2"> 
    <div class="flex flex-row h-8 relative">
      <div class="text-lg"> {{commentIndex}}. {{commentInfo.user_name}} </div>
      <div class="absolute left-24 text-sm"> &gt;&gt; 
        <span class="ml-1" v-for="(one_reply, index) in replyFrom" :key="index">
          {{one_reply}}
        </span>
      </div>
    </div>
    <textarea class="resize-none h-16 w-full" 
      :value="comment"
      @input="$emit('update:comment', $event.target.value)"
    ></textarea>
    <div class="relative h-8">
      <div class="text-xs mt-3 text-white absolute bottom-0 left-0"> {{convert_datetime(commentInfo.timestamp)}} </div>
      <div> <button class="rounded bg-red-400 active:bg-red-600 text-white text-sm absolute bottom-0 right-0 mr-2"
        @click="updateComment()"
      > 更新 </button> </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType, SetupContext, ref, Ref, reactive} from 'vue';
import {TodoCommentInterface} from './models';
import {convert_datetime} from './date';

type Props = {
  commentInfo: TodoCommentInterface,
  commentIndex: number,
  replyFrom: number[],
  comment: string
}

export default defineComponent({
  name: "TodoComment",
  props: {
    commentInfo: {
      type: Object as PropType<TodoCommentInterface>,
      required: true,
    },
    commentIndex: {
      type: Number,
      required: true
    },
    replyFrom: {
      type: Array as PropType<number[]>,
      required: true,
    },
    comment: { // 双方向バインディング
      type: String,
      required: true,
    }
  },
  setup(props: Props, context: SetupContext) {
    // サーバー送信用のデータ


    // 更新ボタンイベント(update:commentとは違うことに注意)
    const updateComment = ():void => {
      context.emit("update-comment", props.commentIndex);  // どのコメントが更新ボタンを押されたか
    }; 


    return {
      updateComment,
      convert_datetime
    };
  },
})
</script>

