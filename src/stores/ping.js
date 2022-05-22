// @ts-check
import { defineStore } from "pinia";
import { ping } from "did_planning_poker";

import { useStore as useIdStore } from "./id";

export const useStore = defineStore({
  id: "ping",
  state: () => ({
    /** @type {number} */
    elapsed: 0,
  }),
  getters: {
    /**
     * @returns { number }
     */
    ping: (state) => state.elapsed,
  },
  actions: {
    /**
     * send ping
     */
    sendPing(did, host) {
      ping(useIdStore().key, did, host)
        .then((value) => (this.elapsed = value))
        .catch(console.error);
    },

    /**
     * update ping response
     * @param {number} elapsed
     */
    receivePong(elapsed) {
      this.elapsed = elapsed;
    },
  },
});
