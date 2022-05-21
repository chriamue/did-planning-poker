<script setup>
import { ref } from "vue";
import PingComp from "./PingComp.vue";
const host = import.meta.env.VITE_MEDIATOR;
const didcomm_host = `${host}/didcomm`;
const did = ref("");
const invitation_str = ref("");
const invitation = fetch(`${host}/invitation`)
  .then(async (r) => (await r.json()).invitation)
  .then((invitation) => {
    did.value = invitation.services[0].recipientKeys[0];
    invitation_str.value = JSON.stringify(invitation);
  });
</script>

<template>
  <div class="invitation">
    <h3>{{ did }}</h3>
    <textarea v-model="invitation_str" cols="100" rows="6"> </textarea>
    <Suspense>
      <ping-comp v-if="invitation" :host="didcomm_host" :did="did" />

      <!-- loading state via #fallback slot -->
      <template #fallback> Loading... </template>
    </Suspense>
  </div>
</template>

<style scoped></style>
