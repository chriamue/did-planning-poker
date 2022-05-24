<script setup>
import { ref } from "vue";
import { useStore } from "@/stores/session";
const host = import.meta.env.VITE_MEDIATOR;
const alias = ref("anonymous host");
const cards = ref("0,1/2,1,2,3,5,8,13,20,40,100");
const store = useStore();
const newSession = () => {
  store
    .newSession(alias, cards.value.split(","), host)
    .then(() => store.startHandler());
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
        v-model="alias"
      />
      <small id="alias_help" class="form-text text-muted"
        >How do you want be recognized?</small
      >
    </div>
    <div class="form-group">
      <label for="host_input">Host Url</label>
      <input id="host_input" type="text" class="form-control" v-model="host" />
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
