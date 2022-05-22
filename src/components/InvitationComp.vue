<script setup>
import { useStore } from "@/stores/session";
const store = useStore();
const copySessionInvitation = () => {
  navigator.clipboard.writeText(store.sessionInvitation);
};
</script>

<template>
  <div class="invitation" v-if="store.id">
    <h3>{{ did }}</h3>
    Share this link to join the Session:
    <textarea
      @click="copySessionInvitation"
      v-model="store.sessionInvitation"
      cols="100"
      rows="6"
    >
    </textarea>
  </div>
</template>

<style scoped></style>
