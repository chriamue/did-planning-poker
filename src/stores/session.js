// @ts-check
import { defineStore } from "pinia";
import { v4 as uuidv4 } from "uuid";
import { did_from_b58 } from "did_planning_poker";
import { useStore as useIdStore } from "./id";

export const useStore = defineStore({
  id: "session",
  state: () => ({
    /** @type {string} */
    m_id: undefined,
    /** @type {string} */
    mediator_did: undefined,
    /** @type {string} */
    m_invitation_str: undefined,
    /** @type {object} */
    m_invitation: undefined,
    /** @type {string} */
    m_host: "",
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
      return this.mediator_did;
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
        mediator_did: this.mediator_did,
      };
      let session_json = JSON.stringify(session);
      return `${window.location.protocol}//${window.location.host}${
        window.location.pathname
      }?join=${btoa(session_json)}`;
    },
  },
  actions: {
    /**
     * generate new session
     * @param {string} mediator_host
     */
    newSession(mediator_host) {
      this.m_host = mediator_host;
      fetch(`${mediator_host}/invitation`)
        .then(async (r) => (await r.json()).invitation)
        .then((invitation) => {
          this.m_id = uuidv4();
          this.mediator_did = invitation.services[0].recipientKeys[0];
          this.m_invitation_str = JSON.stringify(invitation);
          this.m_invitation = invitation;
        });
    },
    joinSession(joinParameter) {
      let session = JSON.parse(atob(joinParameter));
      this.m_id = session.id;
      this.m_host = session.host;
      this.mediator_did = session.mediator_did;
    },
  },
});