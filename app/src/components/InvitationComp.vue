<script setup>
const invitation = await fetch(`http://127.0.0.1:8000/invitation`).then(async (r) => JSON.stringify((await r.json()).invitation))
</script>

<template>
  <div class="invitation">
    <textarea v-model="invitation" cols="100" rows="6">
    </textarea>
  </div>
</template>

<style scoped>
</style>
