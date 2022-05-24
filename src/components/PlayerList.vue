<script setup>
import { useStore } from "@/stores/players";
const store = useStore();
const ping = (ping) => {
  return Math.floor(performance.now() - ping);
};
</script>

<template>
  <div class="player-list">
    <div
      class="card"
      style="width: 18rem"
      v-bind:key="index"
      v-for="(player, index) in store.players"
    >
      <div class="card-header">
        {{ player.alias }}
      </div>
      <div class="card-body">
        <p>
          {{ player.did }}
        </p>
        <p>
          {{ ping(player.ping) }}
        </p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.player-list {
  display: flex;
}
</style>
