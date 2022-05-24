<script setup>
import { ref } from "vue";
import { useStore } from "@/stores/players";
import { useStore as useSessionStore } from "@/stores/session";
import { useStore as useIdStore } from "@/stores/id";
const store = useStore();
const sessionStore = useSessionStore();
const details = ref(false);
const ping = (ping) => {
  return Math.floor(performance.now() - ping);
};
const voted = (player) => {
  return player.did == useIdStore().did
    ? player.voted
    : sessionStore.reveal
    ? player.voted
    : "?";
};
</script>

<template>
  <div class="player-list">
    <div
      class="card"
      style="min-width: 8rem"
      v-bind:key="index"
      v-for="(player, index) in store.players"
    >
      <div class="card-header" @click="details = !details">
        {{ player.alias }}
      </div>
      <div class="card-body">
        <div v-if="details">
          <p>
            {{ player.did }}
          </p>
          <p>
            {{ ping(player.ping) }}
          </p>
        </div>
        <p class="voted">
          {{ voted(player) }}
        </p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.player-list {
  display: flex;
}
.voted {
  text-align: center;
  font-size: xx-large;
}
</style>
