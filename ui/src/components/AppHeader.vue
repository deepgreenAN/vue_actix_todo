<template>
  <div class="sticky top-0 h-20 w-full flex flex-row bg-teal-300 z-10">
    <div class="flex flex-row ml-16">
      <img class="w-24" src="img/logo_svg.svg">
      <img class="w-36" src="img/title.svg">
    </div>
    <div class="flex flex-row mt-10 ml-12">
      <div class="h-8 w-40 text-right mr-2"> ユーザー名 </div>
      <input class="h-8 w-40 overflow-x-auto" type=text maxlength=10
        :value="userName"
        @input="$emit('update:user-name', $event.target.value)" 
      >
    </div>
  </div>
</template>

<script lang=ts>
import { defineComponent, ref, SetupContext} from 'vue';


export default defineComponent({
  name: "AppHeader",
  props: {
    userName: {
      type: String as () => string | null,
      required: false,
      default: null
    }
  }
})
</script>

