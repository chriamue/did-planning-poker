<script setup>
import { ref } from "vue";
import { useStore } from "@/stores/session";
const store = useStore();
const hide = ref(false);
const copySessionInvitation = () => {
  navigator.clipboard.writeText(store.sessionInvitation);
};
</script>

<template>
  <div class="invitation" v-if="store.id">
    <button type="button" class="btn btn-info" @click="hide = !hide">
      Hide invitation
    </button>
    <div class="share-link" v-if="!hide">
      <p>Share this link to join the Session:</p>
      <textarea
        @click="copySessionInvitation"
        v-model="store.sessionInvitation"
        cols="80"
        rows="6"
      >
      </textarea>
    </div>
  </div>
</template>

<style scoped>
.invitation {
  min-height: 80px;
  display: flex;
  overflow: hidden;
  padding: 10px;
}
.invitation button {
  position: absolute;
  right: 20px;
}
</style>
