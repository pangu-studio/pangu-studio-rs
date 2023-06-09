import { DnsProvider, listDnsProvider } from './../api/sslcert'
import { defineStore } from 'pinia'
// import { da } from 'element-plus/es/locale';
export const useSslCertStore = defineStore('sslcert', {
  state: () => ({
    _list: [] as DnsProvider[],
  }),
  getters: {
    list: (state) => state._list,
  },
  actions: {
    async listDnsProvider() {
      this._list = await listDnsProvider()
    },
  },
})
