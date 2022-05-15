<script setup>
import {ref} from 'vue';
import init, {generate_private_key, ping} from "did_planning_poker";
const props = defineProps({
  host: String,
  did: String
})
const key = ref("");
const ping_ms = ref(undefined);
init().then(async () => {
  let private_key = generate_private_key();
  console.log(props.did, props.host);
  ping(private_key, props.did, props.host).then(value => ping_ms.value = value);
  key.value = private_key;
});
</script>

<template>
  <div class="ping">
    {{key}}
    {{ping_ms}}
  </div>
</template>

<style scoped>
</style>
