<script setup>
import { useStore } from "@/stores/session";
import PingComp from "./PingComp.vue";
import PlayerList from "./PlayerList.vue";
import CardList from "./CardList.vue";
import NewSessionComp from "./NewSessionComp.vue";
import SessionDetailsComp from "./SessionDetailsComp.vue";
const store = useStore();
const reveal = () => {
  store.setReveal(!store.reveal);
  store.sendReveal();
};
</script>

<template>
  <div class="session" v-if="store.id">
    <Suspense>
      <ping-comp :host="store.host" :did="store.mediator_did" />
      <template #fallback> Loading... </template>
    </Suspense>
    <session-details-comp />
    <player-list />
    <button type="button" class="btn btn-secondary" @click="reveal">
      Reveal or Hide
    </button>
    <card-list />
  </div>
  <div v-else>
    <new-session-comp />
  </div>
</template>

<style scoped></style>
