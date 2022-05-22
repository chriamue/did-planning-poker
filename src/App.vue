<script setup>
import InvitationComp from "@/components/InvitationComp.vue";
import SessionComp from "@/components/SessionComp.vue";
import JoinComp from "@/components/JoinComp.vue";
const params = new URLSearchParams(document.location.search);
const join = params.get("join");
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
    <join-comp :join="join" v-if="join" />
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
