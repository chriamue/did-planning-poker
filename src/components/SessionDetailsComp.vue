<script setup>
import { ref } from "vue";
import { useStore } from "@/stores/session";
const store = useStore();
const details = ref(false);
</script>

<template>
  <div class="session-details">
    <div v-if="details">
      <input readonly v-model="store.id" />
      <input readonly v-model="store.did" />
      <input readonly v-model="store.host" />
    </div>
    <button type="button" class="btn btn-info" @click="details = !details">
      Session Details
    </button>
  </div>
</template>

<style scoped>
.session-details {
  min-height: 80px;
  display: flex;
  overflow: hidden;
  padding: 10px;
}
.session-details button {
  position: absolute;
  right: 20px;
}
</style>
