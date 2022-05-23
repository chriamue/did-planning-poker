import { defineStore } from "pinia";

export const useStore = defineStore({
  id: "players",
  state: () => ({
    /** @type {object[]} */
    rawItems: [],
  }),
  getters: {
    /**
     * @returns {{ did: string; alias: string, ping: number, voted: string }}
     */
    players: (state) => state.rawItems,
  },
  actions: {
    /**
     * Add player
     * @param {{ did: string; alias: string, ping: number, voted: string }} player
     */
    addPlayer(player) {
      this.rawItems.push(player);
    },

    /**
     * Remove player
     * @param {string} did
     */
    removePlayer(did) {
      const i = this.rawItems
        .map(function (x) {
          return x.did;
        })
        .indexOf(did);
      if (i > -1) this.rawItems.splice(i, 1);
    },

    /**
     * Remove player
     * @param {string} did
     */
    updatePing(did) {
      try {
        const i = this.rawItems
          .map(function (x) {
            return x.did;
          })
          .indexOf(did);
        this.rawItems[i].ping = performance.now();
      } catch (error) {
        console.error(error);
      }
    },
  },
});
