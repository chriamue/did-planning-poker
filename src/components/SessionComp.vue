<script setup>
import { useStore } from "@/stores/session";
import { useStore as usePlayersStore } from "@/stores/players";
import PingComp from "./PingComp.vue";
import PlayerList from "./PlayerList.vue";
import CardList from "./CardList.vue";
import NewSessionComp from "./NewSessionComp.vue";
import SessionDetailsComp from "./SessionDetailsComp.vue";
const sessionStore = useStore();
const playersStore = usePlayersStore();
const reveal = () => {
  sessionStore.setReveal(!sessionStore.reveal);
  sessionStore.sendReveal();
};
const clear = () => {
  playersStore.clearCards();
  sessionStore.setReveal(false);
  sessionStore.sendReveal();
  playersStore.sendPlayers();
};
</script>

<template>
  <div class="session" v-if="sessionStore.id">
    <Suspense>
      <ping-comp :host="sessionStore.host" :did="sessionStore.mediator_did" />
      <template #fallback> Loading... </template>
    </Suspense>
    <session-details-comp />
    <player-list />
    <div class="buttons">
      <button type="button" class="btn btn-secondary" @click="reveal">
        Reveal or Hide
      </button>
      <button type="button" class="btn btn-secondary" v-if="sessionStore.isHost" @click="clear">
        Clear Votes
      </button>
    </div>

    <card-list />
  </div>
  <div v-else>
    <new-session-comp />
  </div>
</template>

<style scoped>
.buttons {
  display: flex;
  margin: 10px;
}
.buttons button {
  margin: 5px;
}
</style>
