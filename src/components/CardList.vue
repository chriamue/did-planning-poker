<script setup>
import { useStore } from "@/stores/session";
const store = useStore();
</script>

<template>
  <div class="card-list">
    <div class="card" v-bind:key="index" v-for="(card, index) in store.cards">
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
</style>
