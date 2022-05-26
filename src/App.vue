<script setup>
import InvitationComp from "@/components/InvitationComp.vue";
import SessionComp from "@/components/SessionComp.vue";
import NewSessionComp from "@/components/NewSessionComp.vue";
import JoinComp from "@/components/JoinComp.vue";
import { useStore } from "./stores/session";
const session = useStore();
const params = new URLSearchParams(document.location.search);
const join = params.get("join");
const logout = () => {
  session.logout();
};
</script>

<template>
  <header>
    <h1>Did Planning Poker</h1>
    <button
      v-if="session.id"
      type="button"
      class="btn btn-danger"
      @click="logout"
    >
      Logout
    </button>
  </header>

  <main>
    <Suspense>
      <invitation-comp />
      <template #fallback> Loading... </template>
    </Suspense>
    <join-comp :join="join" v-if="join" />
    <div v-else>
      <session-comp v-if="session.id" />
      <new-session-comp v-else />
    </div>
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
header button {
  position: absolute;
  top: -10px;
  right: 20px;
}
</style>
