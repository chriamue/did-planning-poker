<script setup>
import PingComp from './PingComp.vue';
const invitation = await fetch(`http://127.0.0.1:8000/invitation`).then(async (r) => (await r.json()).invitation)
const did = invitation.services[0].recipientKeys[0];
const invitation_str = JSON.stringify(invitation);
</script>

<template>
  <div class="invitation">
    <h3>{{did}}</h3>
    <textarea v-model="invitation_str" cols="100" rows="6">
    </textarea>
        <Suspense>
      <ping-comp v-if="invitation" host="http://127.0.0.1:8000/didcomm" :did="did" />

      <!-- loading state via #fallback slot -->
      <template #fallback> Loading... </template>
    </Suspense>
  </div>
</template>

<style scoped>
</style>
