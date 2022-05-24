<script setup>
import { useStore as useSessionStore } from "@/stores/session";
import { useStore as useIdStore } from "@/stores/id";
import { useStore as usePlayersStore } from "@/stores/players";
const store = useSessionStore();
const vote = (card) => {
  usePlayersStore().setVote(useIdStore().did, card);
  usePlayersStore().sendVote(card);
};
</script>

<template>
  <div class="card-list">
    <div
      class="card"
      @click="() => vote(card)"
      v-bind:key="index"
      v-for="(card, index) in store.cards"
    >
      <div class="card-body">
        {{ card }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.card-list {
  display: flex;
}
.card {
  margin: 10px;
}
</style>
