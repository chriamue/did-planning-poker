<script setup>
import { ref } from "vue";
import { useStore } from "@/stores/session";
import PingComp from "./PingComp.vue";
import PlayerList from "./PlayerList.vue";
import CardList from "./CardList.vue";
const host = import.meta.env.VITE_MEDIATOR;
const alias = ref("anonymous host");
const cards = ref("0,1/2,1,2,3,5,8,13,20,40,100");
const store = useStore();
const details = ref(false);
const newSession = () => {
  store
    .newSession(alias, cards.value.split(","), host)
    .then(() => store.startHandler());
};
const reveal = () => {
  store.setReveal(!store.reveal);
  store.sendReveal();
};
</script>

<template>
  <div class="session" v-if="store.id">
    <Suspense>
      <ping-comp :host="store.host" :did="store.mediator_did" />
      <template #fallback> Loading... </template>
    </Suspense>
    <div v-if="details">
      <input readonly v-model="store.id" />
      <input readonly v-model="store.did" />
      <input readonly v-model="store.host" />
    </div>
    <button type="button" class="btn btn-info" @click="details = !details">
      Session Details
    </button>
    <player-list />
    <button type="button" class="btn btn-secondary" @click="reveal">
      Reveal or Hide
    </button>
    <card-list />
  </div>
  <div class="session form-group" v-else>
    <label for="alias_input">Alias</label>
    <input id="alias_input" type="text" v-model="alias" />
    <label for="host_input">Host Url</label>
    <input id="host_input" type="text" v-model="host" />
    <label for="cards_input">Cards</label>
    <input id="cards_input" type="text" v-model="cards" />
    <button type="button" class="btn btn-primary" @click="newSession">
      New Session
    </button>
  </div>
</template>

<style scoped></style>
