import { defineStore } from "pinia";
import { send_players, send_vote } from "did_planning_poker";

import { useStore as useIdStore } from "./id";
import { useStore as useSessionStore } from "./session";

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
        this.rawItems[i].ping = Math.floor(performance.now());
      } catch (error) {
        console.error(error);
      }
    },
    /**
     * Remove player
     * @param {string} did
     * @param {string} vote
     */
    setVote(did, vote) {
      try {
        const i = this.rawItems
          .map(function (x) {
            return x.did;
          })
          .indexOf(did);
        this.rawItems[i].voted = vote;
      } catch (error) {
        console.error(error);
      }
    },

    /**
     * Remove player
     * @param {Array<{ did: string; alias: string, ping: number, voted: string }>} players
     */
    updatePlayers(players) {
      players.forEach((player) => {
        try {
          const i = this.rawItems
            .map(function (x) {
              return x.did;
            })
            .indexOf(player.did);
          if (i == -1) {
            this.addPlayer(player);
          } else {
            this.rawItems[i] = player;
          }
        } catch (error) {
          console.error(error);
        }
      });
    },

    /**
     * send Players
     */
    sendPlayers() {
      this.players.forEach((player) => {
        send_players(
          useSessionStore().id,
          this.players,
          useIdStore().key,
          player.did,
          useSessionStore().mediator_did,
          `${useSessionStore().host}/didcomm`
        ).catch(console.error);
      });
    },
    /**
     * send Players
     */
    sendVote(vote) {
      send_vote(
        useSessionStore().id,
        vote,
        useIdStore().key,
        useSessionStore().did,
        useSessionStore().mediator_did,
        `${useSessionStore().host}/didcomm`
      ).catch(console.error);
    },
  },
});
