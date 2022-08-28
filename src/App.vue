<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import CopyList from './components/CopyList.vue'
import { onMounted, ref, reactive } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
interface wrapper {
  vector:detail[],
  index:number
}
interface detail {
  name:string,
  list:string[]
}
const obj = ref<detail[]>([])
const test = ref('')

//バックエンドからデータを取得
const readData = async () => {
  const data:wrapper = await invoke('read_data')
  obj.value = data['vector']
  name.value = obj.value[data['index']]['name']
}

onMounted(readData)

//jsonを保存

//リストバッファ
const newText = ref('')

//リスト選択
const name = ref('')
const select = async (i:number) => {
  name.value = await obj.value[i]['name']
  await invoke('selected_index', {index: i})
}

//オブジェクト追加 削除
const addObj =async () => {
  if (submitFlag.value == true) {
    await invoke('add_obj', {name: name.value})
    readData()
    newName.value = false
    submitFlag.value = false
  }
}
const removeObj = async () => {
  await invoke('remove_obj')
  readData()
}

const backToDefault = ()=>{
  readData()
  newName.value = false
}


//リスト編集
const addList = async () => {
  if (submitFlag.value == true) {
    await invoke('add_list_text', {text: newText.value})
    readData()
    newText.value = ''
    submitFlag.value = false
  }
}
const removeList = async (textIndex: number, index: number) => {
  await select(index)
  await invoke('remove_list_text', {index: textIndex})
  readData()
}

//オブジェクト追加用フォーム
const newName = ref(false)
const openForm = () => {
  newName.value = true
  name.value = ''
}

const submitFlag = ref(false)
const enableSubmit = ()=>{
  submitFlag.value = true
}
</script>

<template>
<div v-if="newName === false">
  <button @click="openForm">新規</button>
  <button @click="removeObj">削除</button>
  <p>selected: {{name}}</p>
</div>
<div v-else>
  <button @click="backToDefault">戻る</button>
  <input type="text" v-model="name" @keypress.prevent.enter.exact="enableSubmit" @keyup.prevent.enter.exact="addObj" placeholder="new" />
</div>
<input v-if="newName === false" type="text" v-model="newText" @keypress.prevent.enter.exact="enableSubmit" @keyup.prevent.enter.exact="addList" placeholder="text" />
<ul class="list">
  <li class="content" v-for="(detail, i) in obj" v-bind:key="i">
    <h2 @click="select(i)">{{detail["name"]}}</h2>
    <CopyList v-bind:list="detail['list']" v-bind:index="i" @removeList="removeList" />
  </li>
</ul>
</template>


<style scoped>
.list {
  display: flex;
  justify-content: space-around;
  width: 100%;
  flex-wrap: wrap;
}
.content{
  padding-left: 4rem;
  padding-right: 4rem;
  flex-grow: 1;
  min-width: fit-content;
  max-width: 50%;
}
</style>
