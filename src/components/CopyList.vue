<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { writeText} from '@tauri-apps/api/clipboard'
import {ref, toRefs} from 'vue'

const props = defineProps<{
  list: string[],
  index: number
}>()

const { list } = toRefs(props);

const emit = defineEmits<{
  (e: 'removeList', textIndex: number, index: number): void
}>()

const clickRemove = (textIndex: number) => emit('removeList', textIndex, props.index)

const copyFlag = ref(false)
const copyContent = ref('')

async function copy_text(index: number){
  await writeText(list.value[index])
  copyContent.value = list.value[index]
  copyFlag.value = true
  setTimeout(function(){
    copyFlag.value = false
  },3000)
}
</script>

<template>
  <Transition class="notifi">
    <p v-if="copyFlag">クリップボードにコピーしました: {{copyContent}}</p>
  </Transition>
  <ul class="list">
    <li class="copy" v-for="(text, i) in list" v-bind:key="i">
    <span class="text" @click="copy_text(i)">{{text}}</span> <span class="remove" @click="clickRemove(i)">x</span>
    </li>
  </ul>
</template>

<style scoped>
.notifi{
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  color: #1a1a1a;
  background-color: aquamarine;
}
.v-enter-active,
.v-leave-active {
  transition: opacity 0.5s ease;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}
.list {
  display: block;
  width: 100%;
}
.copy {
  width: 100%;
  display: flex;
  justify-content: flex-start;
  text-align: left;
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
  padding-left: 1rem;
}
.text:hover{
  background-color: #1a1a1a;
}
.remove {
  padding-left: 1rem;
  padding-right: 1rem;
}
.remove:hover {
  background-color: #1a1a1a;
}
@media (prefers-color-scheme: light) {
  .text:hover{
    background-color: #bebebe;
  }
}
</style>
