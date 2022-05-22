<script setup>
import { ref } from "vue";
import { useStore } from "@/stores/session";
import PingComp from "./PingComp.vue";
const host = import.meta.env.VITE_MEDIATOR;
const store = useStore();
const details = ref(false);
let params = new URLSearchParams(document.location.search);
let join = params.get("join");
if (join) {
  store.joinSession(join);
}
</script>

<template>
  <div class="session" v-if="store.id">
    <Suspense>
      <ping-comp v-if="store.invitation" :host="store.host" :did="store.did" />
      <template #fallback> Loading... </template>
    </Suspense>
    <div v-if="details">
      <input readonly v-model="store.id" />
      <input readonly v-model="store.did" />
      <input readonly v-model="store.host" />
    </div>
    <button type="button" class="btn btn-info" @click="details = !details">
      Session Details
    </button>
  </div>
  <div class="session form-group" v-else>
    <label for="host_input">Host Url</label>
    <input id="host_input" type="text" v-model="host" />
    <button
      type="button"
      class="btn btn-primary"
      @click="store.newSession(host)"
    >
      New Session
    </button>
  </div>
</template>

<style scoped></style>
