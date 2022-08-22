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
    <li v-for="(text, i) in list" v-bind:key="i">
    <span @click="copy_text(i)">{{text}}</span> <span @click="clickRemove(i)">x</span>
    </li>
  </ul>
</template>

<style scoped>
</style>
