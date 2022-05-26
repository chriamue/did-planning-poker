<script setup>
import { ref } from "vue";
import { useStore as useSessionStore } from "@/stores/session";
import { useStore as useIdStore } from "../stores/id";
const cards = ref("0,1/2,1,2,3,5,8,13,20,40,100,?");
const sessionStore = useSessionStore();
const idStore = useIdStore();
const newSession = () => {
  sessionStore
    .newSession(
      idStore.alias,
      idStore.icon,
      cards.value.split(","),
      sessionStore.host
    )
    .then(() => sessionStore.startHandler());
};
</script>
<template>
  <div class="new-session form">
    <div class="form-group">
      <label for="alias_input">Alias</label>
      <input
        id="alias_input"
        type="text"
        class="form-control"
        v-model="idStore.m_alias"
      />
      <small id="alias_help" class="form-text text-muted"
        >How do you want be recognized?</small
      >
    </div>
    <div class="form-group">
      <label for="icon_input">Icon</label>
      <input
        id="icon_input"
        type="text"
        class="form-control"
        v-model="idStore.m_icon"
      />
      <small id="alias_help" class="form-text text-muted"
        >Nice icon for your card</small
      >
      <div><img width="200" height="200" :src="idStore.icon" /></div>
    </div>
    <div class="form-group">
      <label for="host_input">Host Url</label>
      <input
        id="host_input"
        type="text"
        class="form-control"
        v-model="sessionStore.m_host"
      />
      <small id="alias_help" class="form-text text-muted"
        >Url to mediator.</small
      >
    </div>
    <div class="form-group">
      <label for="cards_input">Cards</label>
      <input
        id="cards_input"
        type="text"
        class="form-control"
        v-model="cards"
      />
      <small id="alias_help" class="form-text text-muted"
        >Comma separate the cards you want to play with.</small
      >
    </div>
    <button type="button" class="btn btn-primary" @click="newSession">
      Start New Session
    </button>
  </div>
</template>

<style scoped></style>
