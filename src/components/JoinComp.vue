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
const id = session.id;
const host = session.host;
const mediator_did = session.mediator_did;
const idStore = useIdStore();
const joinSession = () => {
  store.joinSession(props.join, idStore.alias, idStore.icon);
  store.startHandler();
};
</script>

<template>
  <div class="join" v-if="session && !useStore().id">
    <input readonly v-model="id" />
    <input readonly v-model="host" />
    <input readonly v-model="mediator_did" />
    <input id="alias_input" type="text" v-model="idStore.m_alias" />
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
</template>

<style scoped></style>
