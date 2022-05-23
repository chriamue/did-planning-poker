// @ts-check
import { defineStore } from "pinia";
import { send_ping, send_pong } from "did_planning_poker";

import { useStore as useIdStore } from "./id";
import { useStore as useSessionStore } from "./session";

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
    sendPing() {
      send_ping(
        useIdStore().key,
        useSessionStore().did,
        useSessionStore().mediator_did,
        `${useSessionStore().host}/didcomm`
      )
        .then((id) => this.timestamps.set(id, performance.now()))
        .catch(console.error);
    },

    /**
     * send ping
     * @param {string} did
     * @param {string} thid
     */
    sendPong(did, thid) {
      send_pong(
        thid,
        useIdStore().key,
        did,
        useSessionStore().mediator_did,
        `${useSessionStore().host}/didcomm`
      ).catch(console.error);
    },

    /**
     * update ping response
     * @param {string} thid
     */
    receivePong(did, thid) {
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
