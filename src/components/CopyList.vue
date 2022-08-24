<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { writeText} from '@tauri-apps/api/clipboard'
import {ref, toRefs} from 'vue'

const props = defineProps<{
  list: string[],
  url: string,
  index: number
}>()

const { list } = toRefs(props);

const emit = defineEmits<{
  (e: 'removeList', textIndex: number, index: number): void
}>()

const clickRemove = (textIndex: number) => emit('removeList', textIndex, props.index)

async function copy_text(index: number){
  await writeText(list.value[index])
}
</script>

<template>
  <ul>
    <li class="copy" v-for="(text, i) in list" v-bind:key="i">
    <span class="text" @click="copy_text(i)">{{text}}</span> <span @click="clickRemove(i)">x</span>
    </li>
  </ul>
</template>

<style scoped>
.copy {
  display: flex;
  justify-content: flex-start;
}
.copy:first-child {
  border-top: 1px solid #666;
  border-bottom: 1px solid #666;
}
.copy + .copy{
  border-bottom: 1px solid #666;
}
.text {
  flex-grow: 1;
}
.text:hover{
  background-color: #1a1a1a;
}
@media (prefers-color-scheme: light) {
  .text:hover{
    background-color: #bebebe;
  }
}
</style>
