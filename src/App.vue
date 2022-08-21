<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import CopyList from './components/CopyList.vue'
import {onMounted, reactive, ref} from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
const obj:{[key: string]: {[key: string]}} = reactive({})
const test = ref('')

//ファイルからjsonを取得
const getJson = async () => {
  const result = await invoke('get_json')as string;
  test.value = result
  const jsonObj = JSON.parse(test.value)
  for(const i in jsonObj){
    obj[i] = jsonObj[i]
  }
}
onMounted(getJson)

//jsonを保存
const saveJson =async () => {
  const deserialized = JSON.stringify(obj)
  await invoke('save_json', { data: deserialized})
}

//リストバッファ
const newText = ref('')

//リスト選択
const name = ref('')
const select = (key :string|number) => {
  const l = key as string
  name.value = l
}

//オブジェクト削除
const removeObj = () => {
  if (name.value !== ''){
    delete obj[name.value]
    saveJson()
  }
}


//リスト編集
const addList = () => {
  if (name.value !== ''){
    if (name.value in obj){
      obj[name.value]['list'].push(newText.value)
    } else {
      obj[name.value] = {url: '', list: []}
    }
    saveJson()
    newText.value = ''
    newName.value = false
  }
}
const removeList = (index: number, str: string) => {
  name.value = str
  obj[name.value]['list'].splice(index,1)
  saveJson()
}

//オブジェクト追加用フォーム
const newName = ref(false)
const openForm = () => {
  newName.value = true
  name.value = ''
}
</script>

<template>
<p>推しぴ</p>
<p>selected: {{name}}</p>
<div v-if="newName === false"><button @click="openForm">新規</button><button @click="removeObj">削除</button></div>
<input v-else type="text" v-model="name" placeholder="new" />
<button @click="addList">追加</button>
<input v-if="newName === false" type="text" v-model="newText" placeholder="text" />
<ul>
  <li v-for="(value, name) in obj">
    <p @click="select(name)">{{name}}</p>
    <CopyList v-bind:list="value['list']" v-bind:name="(name as string)" @removeList="removeList" />
  </li>
</ul>
</template>


<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
