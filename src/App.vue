<script setup>
import { ref } from "vue";
import InvitationComp from "./components/InvitationComp.vue";

import init, {init_panic_hook} from "did_planning_poker";
const wasm_loaded = ref(false);
init().then(() => {
  init_panic_hook();
  wasm_loaded.value = true;
  console.log("wasm loaded");
});
</script>

<template>
  <header>
    <h1>Did Planning Poker</h1>
  </header>

  <main v-if="wasm_loaded">
    <Suspense>
      <invitation-comp />

      <!-- loading state via #fallback slot -->
      <template #fallback> Loading... </template>
    </Suspense>
  </main>
</template>

<style>
@import "./assets/base.css";

#app {
  max-width: 1280px;
  margin: 0 auto;
  padding: 2rem;

  font-weight: normal;
}

header {
  line-height: 1.5;
}
</style>
