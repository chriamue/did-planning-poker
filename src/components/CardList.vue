<script setup>
import { ref } from "vue";
import { useStore as useSessionStore } from "@/stores/session";
import { useStore as useIdStore } from "@/stores/id";
import { useStore as usePlayersStore } from "@/stores/players";
const store = useSessionStore();
const disabled = ref(undefined);
const vote = (card) => {
  disabled.value = card;

  setTimeout(() => {
    disabled.value = undefined;
  }, 1500);
  usePlayersStore().setVote(useIdStore().did, card);
  usePlayersStore().sendVote(card);
};
</script>

<template>
  <div class="card-list">
    <div
      class="card shadow"
      :class="{ shake: disabled == card }"
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
  text-align: center;
  font-size: x-large;
}
.shake {
  animation: shake 0.82s cubic-bezier(0.36, 0.07, 0.19, 0.97) both;
  transform: translate3d(0, 0, 0);
}

@keyframes shake {
  10%,
  90% {
    transform: translate3d(-1px, 0, 0);
  }

  20%,
  80% {
    transform: translate3d(2px, 0, 0);
  }

  30%,
  50%,
  70% {
    transform: translate3d(-4px, 0, 0);
  }

  40%,
  60% {
    transform: translate3d(4px, 0, 0);
  }
}
</style>
