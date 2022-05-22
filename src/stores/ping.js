// @ts-check
import { defineStore } from "pinia";
import { send_ping } from "did_planning_poker";

import { useStore as useIdStore } from "./id";

export const useStore = defineStore({
  id: "ping",
  state: () => ({
    /** @type {number} */
    elapsed: 0,
    /** @type {Map<string, number>} */
    timestamps: new Map(),
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
      send_ping(useIdStore().key, did, host)
        .then((id) => this.timestamps.set(id, performance.now()))
        .catch(console.error);
    },

    /**
     * update ping response
     * @param {string} thid
     */
    receivePong(thid) {
      let timestamp = this.timestamps.get(thid);
      if (timestamp) {
        let now = performance.now();
        this.elapsed = Math.floor(now - timestamp);
      } else {
        this.elapsed = 10000;
      }
    },
  },
});
