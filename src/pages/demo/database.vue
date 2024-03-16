<script lang="ts" setup>
import Database from 'tauri-plugin-sql-api'

const db = ref<Database>()
const tips = ref('')
const data = ref<any[]>()
const name = ref('')

const columns = [
  { title: 'ID', dataIndex: 'id' },
  { title: 'Name', dataIndex: 'name' },
]

async function handleCreate() {
  await db.value?.execute('insert into users (name) values (?)', [name.value])
  await handleSelect()
}

async function handleSelect() {
  data.value = await db.value?.select('select * from users')
}

async function handleLoad() {
  db.value = await Database.load('sqlite:database.db')
  tips.value = 'Database is loaded'
  await handleSelect()
}
</script>

<template>
  <a-space direction="vertical" size="large" fill>
    <h1 class="text-4xl">
      Database Demo
    </h1>

    <a-space fill>
      <a-button type="primary" :disabled="!!db" @click="handleLoad">
        Load Database
      </a-button>
      <p>{{ tips }}</p>
    </a-space>

    <a-space fill>
      <a-input v-model="name" placeholder="Name" />
      <a-button type="primary" @click="handleCreate">
        Insert
      </a-button>
    </a-space>

    <a-table :columns="columns" :data="data" />
  </a-space>
</template>
