<script setup>
import { ref } from "vue";
import { useStore } from "@/stores/players";
const store = useStore();
const details = ref(false);
const ping = (ping) => {
  return Math.floor(performance.now() - ping);
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
          {{ player.voted }}
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
