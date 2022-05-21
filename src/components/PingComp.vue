<script setup>
import { useStore } from "@/stores/ping";
const props = defineProps({
  host: String,
  did: String,
});
const store = useStore();
const interval = setInterval(async () => {
  store.sendPing(props.did, `${props.host}/didcomm`);
}, 3000);
</script>

<template>
  <div class="ping" v-if="interval">
    {{ store.elapsed }}
  </div>
</template>

<style scoped></style>
