<script setup>
import { ref } from "vue";
import { useStore } from "@/stores/session";
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
const alias = ref("anonymous");
const joinSession = () => {
  store.joinSession(props.join, alias.value);
  store.startHandler();
};
</script>

<template>
  <div class="join" v-if="session && !useStore().id">
    <input readonly v-model="id" />
    <input readonly v-model="host" />
    <input readonly v-model="mediator_did" />
    <input id="alias_input" type="text" v-model="alias" />
    <button type="button" class="btn btn-info" @click="() => joinSession()">
      Join Session
    </button>
  </div>
</template>

<style scoped></style>
