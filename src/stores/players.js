import { defineStore } from "pinia";
import { send_players, send_vote } from "did_planning_poker";

import { useStore as useIdStore } from "./id";
import { useStore as useSessionStore } from "./session";

export const useStore = defineStore({
  id: "players",
  state: () => ({
    /** @type {Array<{ did: string; alias: string, icon: string, ping: number, voted: string }>} */
    m_players: [],
  }),
  getters: {
    /**
     * @returns {Array<{ did: string; alias: string, icon: string, ping: number, voted: string }>}
     */
    players: (state) => state.m_players,
  },
  actions: {
    /**
     * Add player
     * @param {{ did: string; alias: string, icon: string, ping: number, voted: string }} player
     */
    addPlayer(player) {
      this.m_players.push(player);
    },

    /**
     * Remove player
     * @param {string} did
     */
    removePlayer(did) {
      const i = this.m_players
        .map(function (x) {
          return x.did;
        })
        .indexOf(did);
      if (i > -1) this.m_players.splice(i, 1);
    },

    /**
     * Remove player
     * @param {string} did
     */
    updatePing(did) {
      try {
        const i = this.m_players
          .map(function (x) {
            return x.did;
          })
          .indexOf(did);
        this.m_players[i].ping = Math.floor(performance.now());
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
        const i = this.m_players
          .map(function (x) {
            return x.did;
          })
          .indexOf(did);
        this.m_players[i].voted = vote;
      } catch (error) {
        console.error(error);
      }
    },

    /**
     * Remove player
     * @param {Array<{ did: string; alias: string, icon: string, ping: number, voted: string }>} players
     */
    updatePlayers(players) {
      players.forEach((player) => {
        try {
          const i = this.m_players
            .map(function (x) {
              return x.did;
            })
            .indexOf(player.did);
          if (i == -1) {
            this.addPlayer(player);
          } else {
            this.m_players[i] = player;
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
    clearCards() {
      this.players.forEach((player) => {
        player.voted = "";
      });
    },
  },
  persist: true,
});
