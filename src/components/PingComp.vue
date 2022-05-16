<script setup>
import { ref } from "vue";
import init, {
  generate_private_key,
  ping,
  init_panic_hook,
} from "did_planning_poker";
const props = defineProps({
  host: String,
  did: String,
});
const key = ref("");
const ping_ms = ref(undefined);
const interval = setInterval(() => {
  init().then(async () => {
    init_panic_hook();
    let private_key = generate_private_key();
    console.log(props.did, props.host);
    ping(private_key, props.did, props.host).then(
      (value) => (ping_ms.value = value)
    );
    key.value = private_key;
  });
}, 3000);
</script>

<template>
  <div class="ping">
    {{ ping_ms }}
  </div>
</template>

<style scoped>
</style>
