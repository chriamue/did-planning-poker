<script setup>
import { useStore } from "@/stores/session";
import PingComp from "./PingComp.vue";
const host = import.meta.env.VITE_MEDIATOR;
const store = useStore();
store.newSession(host);
</script>

<template>
  <div class="invitation" v-if="store.id">
    <h3>{{ did }}</h3>
    <textarea v-model="store.invitation_str" cols="100" rows="6"> </textarea>
    <Suspense>
      <ping-comp v-if="store.invitation" :host="store.host" :did="store.did" />
      <template #fallback> Loading... </template>
    </Suspense>
  </div>
</template>

<style scoped></style>
