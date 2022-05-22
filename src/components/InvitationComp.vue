<script setup>
import { useStore } from "@/stores/session";
const store = useStore();
</script>

<template>
  <div class="invitation" v-if="store.id">
    <h3>{{ did }}</h3>
    <textarea v-model="store.invitation_str" cols="100" rows="6"> </textarea>
  </div>
</template>

<style scoped></style>
