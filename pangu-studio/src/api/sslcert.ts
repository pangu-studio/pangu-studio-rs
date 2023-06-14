import { P } from '@tauri-apps/api/event-30ea0228'
import { invoke } from '@tauri-apps/api/tauri'

export interface DnsProvider {
  id: number
  name: string
  api_key: string
  api_secret: string
  create_time: string
  update_time: string
}

export interface SSLCertificate {
  id: number
  domains: string
  cert_chain: string
  private_key: string
  status: string
  deleted: boolean
  addition: string
  create_time: string
  update_time: string
}
export interface SSLCertificateCreate {
  sn: string
  provider_id: number
  mail: string
  domain: string
  subdomain: string
}

export interface CreateDnsProvider {
  name: string
  api_key: string
  api_secret: string
  provider_type: string
}

export async function listDnsProvider(): Promise<DnsProvider[]> {
  return invoke('list_dns_providers')
}
export async function listSSLCertificate(): Promise<SSLCertificate[]> {
  return invoke('list_sslcerts')
}
export async function applyCertificate(
  cert: SSLCertificateCreate
): Promise<SSLCertificate> {
  return invoke('apply_certificate', { cert })
}
export async function getSslCertBySN(sn: string): Promise<SSLCertificate> {
  return invoke('get_sslcert_by_sn', { sn })
}
export async function removeSslCert(id: number): Promise<SSLCertificate> {
  return invoke('remove_sslcert', { id })
}

export async function addDnsProvider(
  provider: CreateDnsProvider
): Promise<DnsProvider> {
  return invoke('add_dns_provider', { provider })
}
