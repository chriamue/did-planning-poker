<script setup>
import { ref } from "vue";
import { useStore } from "@/stores/players";
import { useStore as useSessionStore } from "@/stores/session";
import { useStore as useIdStore } from "@/stores/id";
import PlayerDetails from "./PlayerDetails.vue";
const store = useStore();
const sessionStore = useSessionStore();
const details = ref(false);

const voted = (player) => {
  return player.did == useIdStore().did
    ? player.voted
    : sessionStore.reveal
    ? player.voted
    : player.voted == ""
    ? ""
    : "?";
};
</script>

<template>
  <div class="player-list">
    <div
      class="card card-flip"
      :class="{ reveal: sessionStore.reveal }"
      style="min-width: 8rem"
      v-bind:key="index"
      v-for="(player, index) in store.players"
    >
      <div class="card-front">
        <div class="card-header" @click="details = !details">
          <img width="60" height="60" :src="player.icon" />
          {{ player.alias }}
        </div>
        <div class="card-body">
          <player-details :player="player" v-if="details" />
          <p class="voted">
            {{ voted(player) }}
          </p>
        </div>
      </div>
      <div class="card-back">
        <div class="card-header" @click="details = !details">
          <img width="60" height="60" :src="player.icon" />
          {{ player.alias }}
        </div>
        <div class="card-body">
          <player-details :player="player" v-if="details" />
          <p class="voted">
            {{ voted(player) }}
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.player-list {
  display: flex;
}
.card-header {
  vertical-align: middle;
}
.card-header img {
  margin: 10px;
}
.card {
  margin: 5px;
}
.voted {
  text-align: center;
  font-size: xx-large;
}

.card-flip > div {
  backface-visibility: hidden;
  transition: transform 300ms;
  transition-timing-function: linear;
}

.card-front {
  transform: rotateY(0deg);
}

.card-back {
  transform: rotateY(180deg);
  position: absolute;
  top: 0;
}

.reveal .card-front {
  transform: rotateY(-180deg);
}

.reveal .card-back {
  transform: rotateY(0deg);
}
</style>
