import { invoke } from '@tauri-apps/api'

export interface Bindings {
  f5?: string
  f6?: string
  f7?: string
  f8?: string
  f9?: string
  f10?: string
}

export function getBindings(): Promise<Bindings> {
  return invoke('get_bindings')
}

export function bind(key: string, stratagem: string | null): Promise<void> {
  return invoke('bind', { key, stratagem })
}
