<script setup>
import InvitationComp from "@/components/InvitationComp.vue";
import SessionComp from "@/components/SessionComp.vue";
import NewSessionComp from "@/components/NewSessionComp.vue";
import JoinComp from "@/components/JoinComp.vue";
import { useStore } from "./stores/session";
const session = useStore();
session.stopHandler();
const params = new URLSearchParams(document.location.search);
const join = params.get("join");
const logout = () => {
  session.logout();
};
</script>

<template>
  <header>
    <h1>Did Planning Poker</h1>
    <div class="buttons">
      <button
        id="join-button"
        v-if="session.id && !session.interval"
        type="button"
        class="btn btn-success"
        @click="() => session.startHandler()"
      >
        Join
      </button>
      <button
        id="logout-button"
        v-if="session.id"
        type="button"
        class="btn btn-danger"
        @click="logout"
      >
        Logout
      </button>
    </div>
  </header>

  <main>
    <Suspense>
      <invitation-comp />
      <template #fallback> Loading... </template>
    </Suspense>

    <session-comp v-if="session.id" />
    <div v-else>
      <join-comp :join="join" v-if="join" />
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
header .buttons {
  position: absolute;
  top: -10px;
  right: 20px;
}
header button {
  margin: 5px;
}
</style>
