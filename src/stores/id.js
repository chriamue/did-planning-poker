// @ts-check
import { defineStore } from "pinia";
import { did_from_b58, generate_private_key } from "did_planning_poker";

export const useStore = defineStore({
  id: "id",
  state: () => ({
    /** @type {string} */
    private_key: generate_private_key(),
    m_alias: "anonymous",
    m_icon:
      "https://upload.wikimedia.org/wikipedia/commons/7/79/Face-smile.svg",
  }),
  getters: {
    /**
     * @returns { string }
     */
    key() {
      return this.private_key;
    },
    did() {
      return did_from_b58(this.private_key);
    },
    alias() {
      return this.m_alias;
    },
    icon() {
      return this.m_icon;
    },
  },
  actions: {
    /**
     * generate new id
     */
    generate() {
      this.private_key = generate_private_key();
    },

    /**
     * import existing private key
     * @param {string} private_key
     */
    import(private_key) {
      this.private_key = private_key;
    },
    /**
     * update ping response
     * @param {string} alias
     */
    setAlias(alias) {
      this.m_alias = alias;
    },
    /**
     * update ping response
     * @param {string} icon
     */
    setIcon(icon) {
      this.m_icon = icon;
    },
  },
  persist: true,
});
