<script setup>
import InvitationComp from "@/components/InvitationComp.vue";
import SessionComp from "@/components/SessionComp.vue";
</script>

<template>
  <header>
    <h1>Did Planning Poker</h1>
  </header>

  <main>
    <Suspense>
      <invitation-comp />
      <template #fallback> Loading... </template>
    </Suspense>
    <session-comp />
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
