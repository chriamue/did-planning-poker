<script setup>
import { useStore } from "@/stores/session";
import { useStore as useIdStore } from "../stores/id";
const props = defineProps({
  join: String,
});
let session = undefined;
if (props.join) {
  session = JSON.parse(atob(props.join));
}
const store = useStore();
const idStore = useIdStore();
const joinSession = () => {
  store.joinSession(props.join, idStore.alias, idStore.icon);
  store.startHandler();
};
</script>

<template>
  <div class="join" v-if="session && !useStore().id">
    <div class="form-group">
      <div class="host-details form-group">
        <input disabled v-model="session.did" title="Host did" />
        <input disabled v-model="session.id" title="Session id" />
        <input disabled v-model="session.host" title="Mediator Url" />
        <input disabled v-model="session.mediator_did" title="Mediator did" />
      </div>
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
      <button type="button" class="btn btn-info" @click="() => joinSession()">
        Join Session
      </button>
    </div>
  </div>
</template>

<style scoped>
.host-details {
  margin: 10px auto;
}
.host-details input {
  margin: 5px;
}
</style>
