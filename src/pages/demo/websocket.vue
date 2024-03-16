<script lang="ts" setup>
const wsUrl = ref('ws://127.0.0.1:9002')
const input = ref('')
const output = ref('')

const { status, data, send, close, open } = useWebSocket(wsUrl.value, {
  immediate: false,
  autoClose: false,
})

function handleConnect() {
  open()
}

function handleSend() {
  send(input.value)
  output.value += `[client] ${input.value} \n`
  input.value = ''
}

watchEffect(() => {
  if (data.value)
    output.value += `[server] ${data.value} \n`
})
</script>

<template>
  <a-space direction="vertical" size="large" fill>
    <h1 class="text-4xl">
      Websocket Demo
    </h1>
    <a-space fill>
      <a-input v-model="wsUrl" />
      <a-button type="primary" :disabled="status !== 'CLOSED'" @click="handleConnect">
        Connect
      </a-button>
      <a-button type="primary" :disabled="status !== 'OPEN'" status="danger" @click="close()">
        DisConnect
      </a-button>
    </a-space>

    <a-space fill>
      <a-input v-model="input" />
      <a-button type="primary" status="success" :disabled="status !== 'OPEN'" @click="handleSend">
        Send
      </a-button>
    </a-space>

    <a-textarea
      v-model="output" :auto-size="{
        minRows: 8,
      }"
    />
  </a-space>
</template>
