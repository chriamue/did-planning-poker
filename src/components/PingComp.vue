<script setup>
import { useStore } from "@/stores/ping";
const store = useStore();
const interval = setInterval(async () => {
  store.sendPing();
}, 3000);
const destoyed = () => {
  clearInterval(interval);
};
</script>

<template>
  <div class="ping" v-if="interval">
    {{ store.elapsed }}
  </div>
</template>

<style scoped>
.ping {
  position: fixed;
  top: 20px;
  right: 40px;
}
</style>
