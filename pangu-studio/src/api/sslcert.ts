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

export async function listDnsProvider(): Promise<DnsProvider[]> {
  return invoke('list_dns_providers')
}
