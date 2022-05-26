// @ts-check
import { defineStore } from "pinia";
import { send_join, send_cards, send_reveal } from "did_planning_poker";
import { v4 as uuidv4 } from "uuid";
import { did_from_b58, Handler } from "did_planning_poker";
import { useStore as useIdStore } from "./id";
import { useStore as usePingStore } from "./ping";
import { useStore as usePlayersStore } from "./players";
import { useStore as useSessionStore } from "./session";

export const useStore = defineStore({
  id: "session",
  state: () => ({
    /** @type {string} */
    m_id: undefined,
    /** @type {string} */
    m_mediator_did: undefined,
    /** @type {string} */
    m_did: undefined,
    /** @type {string} */
    m_invitation_str: undefined,
    /** @type {object} */
    m_invitation: undefined,
    /** @type {string} */
    m_host: import.meta.env.VITE_MEDIATOR,
    m_handler: undefined,
    m_interval: undefined,
    m_cards: [],
    m_reveal: false,
  }),
  getters: {
    /**
     * @returns { string }
     */
    id() {
      return this.m_id;
    },
    /**
     * @returns { string }
     */
    host() {
      return this.m_host;
    },
    /**
     * @returns { string }
     */
    did() {
      return this.m_did;
    },
    /**
     * @returns { string }
     */
    mediator_did() {
      return this.m_mediator_did;
    },
    /**
     * @returns { string }
     */
    invitation() {
      return this.m_invitation;
    },
    /**
     * @returns { string }
     */
    invitation_str() {
      return this.m_invitation_str;
    },
    sessionInvitation() {
      const idStore = useIdStore();
      let session = {
        id: this.id,
        host: this.m_host,
        did: did_from_b58(idStore.key),
        mediator_did: this.m_mediator_did,
      };
      let session_json = JSON.stringify(session);
      return `${window.location.protocol}//${window.location.host}${
        window.location.pathname
      }?join=${btoa(session_json)}`;
    },
    cards() {
      return this.m_cards;
    },
    reveal() {
      return this.m_reveal;
    },
    isHost() {
      return this.m_did == useIdStore().did;
    },
    interval() {
      return this.m_interval;
    },
    voteCount() {
      return this.m_cards.map((card) => {
        return {
          card: card,
          count: usePlayersStore().players.filter(({ voted }) => voted == card)
            .length,
        };
      });
    },
  },
  actions: {
    /**
     * generate new session
     * @param {string} mediator_host
     */
    newSession(alias, icon, cards, mediator_host) {
      usePlayersStore().m_players = [];
      this.m_cards = cards;
      usePlayersStore().addPlayer({
        did: did_from_b58(useIdStore().key),
        alias,
        icon,
        ping: performance.now(),
        voted: "",
      });
      this.m_host = mediator_host;
      return fetch(`${mediator_host}/invitation`)
        .then(async (r) => (await r.json()).invitation)
        .then((invitation) => {
          this.m_id = uuidv4();
          this.m_did = did_from_b58(useIdStore().key);
          this.m_mediator_did = invitation.services[0].recipientKeys[0];
          this.m_invitation_str = JSON.stringify(invitation);
          this.m_invitation = invitation;
        });
    },
    joinSession(joinParameter, alias, icon) {
      usePlayersStore().m_players = [];
      usePlayersStore().addPlayer({
        did: did_from_b58(useIdStore().key),
        alias,
        icon,
        ping: performance.now(),
        voted: "",
      });
      let session = JSON.parse(atob(joinParameter));
      this.m_id = session.id;
      this.m_host = session.host;
      this.m_did = session.did;
      this.m_mediator_did = session.mediator_did;
      useIdStore().setAlias(alias);
      useIdStore().setIcon(icon);
      send_join(
        session.id,
        alias,
        icon,
        useIdStore().key,
        session.did,
        session.mediator_did,
        session.host + "/didcomm"
      )
        .then((id) => console.log(id))
        .catch(console.error);
    },
    startHandler() {
      let key = useIdStore().private_key;
      let mediator_host = this.m_host + "/didcomm";
      let mediator_did = this.m_mediator_did;
      let handler = new Handler(key, mediator_host, mediator_did);
      handler.on("ping", (value) => {
        usePingStore().sendPong(value.did, value.id);
        usePlayersStore().updatePing(value.did);
      });
      handler.on("ping-response", (value) => {
        usePingStore().receivePong(value.did, value.thid);
        usePlayersStore().updatePing(value.did);
      });
      handler.on("players", (value) => {
        if (useSessionStore().did != useIdStore().did) {
          usePlayersStore().updatePlayers(value.players);
        }
        console.log(value);
      });
      handler.on("cards", (value) => {
        if (useSessionStore().did != useIdStore().did) {
          this.setCards(value.cards);
        }
      });
      handler.on("vote", (value) => {
        usePlayersStore().setVote(value.did, value.vote);
        usePlayersStore().sendPlayers();
      });
      handler.on("reveal", (value) => {
        this.setReveal(value.reveal);
      });
      handler.on("join", (value) => {
        let player = {
          did: value.did,
          alias: value.alias,
          icon: value.icon,
          ping: performance.now(),
          voted: "",
        };
        usePlayersStore().addPlayer(player);
        usePlayersStore().sendPlayers();
        this.sendCards(value.did);
        console.log(value, "joined");
      });
      this.m_handler = handler;
      this.m_interval = setInterval(() => {
        handler.next().then((messages) => handler.handle(messages));
      }, 500);
    },
    stopHandler() {
      if (this.m_interval) {
        clearInterval(this.m_interval);
        this.m_interval = undefined;
      }
    },
    setCards(cards) {
      this.m_cards = cards;
    },
    setReveal(reveal) {
      this.m_reveal = reveal;
    },
    sendCards(did) {
      send_cards(
        useSessionStore().id,
        this.cards,
        useIdStore().key,
        did,
        useSessionStore().mediator_did,
        `${useSessionStore().host}/didcomm`
      ).catch(console.error);
    },
    /**
     * send Reveal
     */
    sendReveal() {
      usePlayersStore().players.forEach((player) => {
        send_reveal(
          useSessionStore().id,
          this.m_reveal,
          useIdStore().key,
          player.did,
          useSessionStore().mediator_did,
          `${useSessionStore().host}/didcomm`
        ).catch(console.error);
      });
    },
    logout() {
      this.m_id = undefined;
    },
  },
  persist: true,
});
