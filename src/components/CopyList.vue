<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { writeText} from '@tauri-apps/api/clipboard'
import {ref} from 'vue'
const newText = ref<string>('')
const texts = ref<string[]>([])
const addText = () => {
  texts.value.push(newText.value)
}
const removeText = (index: number) => texts.value.splice(index,1)

async function copy_text(index: number){
  await writeText(texts.value[index])
}
</script>

<template>
<p>推しぴ</p>
  <input type="text" v-model="newText" placeholder="edit me" />
  <button @click="addText()">追加</button>
  <ul>
    <li v-for="(text, i) in texts" v-bind:key="i">
    <span @click="copy_text(i)">{{text}}</span> <span @click="removeText(i)">x</span>
    </li>
  </ul>
</template>

<style scoped>
</style>
