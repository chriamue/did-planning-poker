<script setup>
import { useStore } from "@/stores/session";
import PingComp from "./PingComp.vue";
const host = import.meta.env.VITE_MEDIATOR;
const store = useStore();
</script>

<template>
  <div class="session" v-if="store.id">
    <Suspense>
      <ping-comp v-if="store.invitation" :host="store.host" :did="store.did" />
      <template #fallback> Loading... </template>
    </Suspense>
  </div>
  <div class="session form-group" v-else>
    <label for="host_input">Host Url</label>
    <input id="host_input" type="text" v-model="host">
    <button type="button" class="btn btn-primary" @click="store.newSession(host)">New Session</button>
  </div>
</template>

<style scoped></style>
