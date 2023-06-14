import {
  DnsProvider,
  listDnsProvider,
  listSSLCertificate,
  SSLCertificate,
  getSslCertBySN,
  removeSslCert,
  CreateDnsProvider,
  addDnsProvider,
} from './../api/sslcert'
import { defineStore } from 'pinia'
// import { da } from 'element-plus/es/locale';
export const useSslCertStore = defineStore('sslcert', {
  state: () => ({
    _list: [] as DnsProvider[],
    _sslcerts: [] as SSLCertificate[],
    _applyCert: {} as SSLCertificate,
  }),
  getters: {
    list: (state) => state._list,
    sslcerts: (state) => state._sslcerts,
    applyCert: (state) => state._applyCert,
    applyCertArray: (state) => {
      if (state._applyCert) {
        return [state._applyCert]
      }
      return []
    },
    applyCertAddtion: (state) => {
      if (state._applyCert.addition) {
        return [JSON.parse(state._applyCert.addition)]
      }
      return []
    },
  },
  actions: {
    async listDnsProvider() {
      this._list = await listDnsProvider()
    },
    async listSSLCertificate() {
      this._sslcerts = await listSSLCertificate()
    },
    async getSslCertBySN(sn: string) {
      this._applyCert = await getSslCertBySN(sn)
    },
    async removeSslCert(id: number) {
      removeSslCert(id).then(() => {
        this._sslcerts = this._sslcerts.filter((item) => item.id !== id)
      })
    },
    async addDnsProvider(provider: CreateDnsProvider) {
      addDnsProvider(provider).then((item) => {
        console.log('add dns provider success id:', item)
      })
    },
  },
})
