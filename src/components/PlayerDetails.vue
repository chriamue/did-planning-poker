<script setup>
import { useStore as usePlayersStore } from "@/stores/players";
import { useStore as useSessionStore } from "@/stores/session";
const props = defineProps({
  player: Object,
});

const sessionStore = useSessionStore();
const playersStore = usePlayersStore();
const ping = (ping) => {
  return Math.floor(performance.now() - ping);
};
const remove = () => {
  playersStore.removePlayer(props.player.did);
};
</script>

<template>
  <div class="player-details">
    <p>
      {{ player.did }}
    </p>
    <p>
      {{ ping(player.ping) }}
    </p>
    <div v-if="sessionStore.isHost">
      <button type="button" class="btn btn-danger" @click="remove">
        Remove Player
      </button>
    </div>
  </div>
</template>

<style scoped>
.player-details {
  margin: 10px;
}
</style>
