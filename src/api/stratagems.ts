import { invoke } from '@tauri-apps/api'
import groupBy from '~/utils/groupBy'

export interface Stratagem {
  name: string
  sequence: string
  icon: string
  group: number
}

export async function getStratagems(): Promise<Stratagem[][]> {
  // const sequences: Record<number, string> = Object.fromEntries(Object.entries(import.meta.glob('../../public/stratagems/*/sequence.txt', { eager: true, import: 'default' }))
  //   .map(([k, v]) => [
  //     Number.parseInt(k.replace(/^.*\/stratagems\/([^/]+)\/sequence\.txt$/, '$1')),
  //     String(v).toUpperCase().replaceAll(/[^UDLR]/g, ''),
  //   ]))
  // const names: Record<number, string> = Object.fromEntries(Object.entries(import.meta.glob('../../public/stratagems/*/name.txt', { eager: true, import: 'default' }))
  //   .map(([k, v]) => [
  //     Number.parseInt(k.replace(/^.*\/stratagems\/([^/]+)\/name\.txt$/, '$1')),
  //     String(v),
  //   ]))

  // const ids = Object.keys(names).map(it => Number.parseInt(it))

  // const entries = ids
  //   .map(id => ({
  //     id,
  //     name: names[id],
  //     sequence: sequences[id],
  //     icon: `stratagems/${`0000${id}`.slice(-4)}/icon.png`,
  //     group: Math.floor(id / 1000),
  //   }))

  // return Object.values(groupBy(entries, it => it.group))
  return invoke('get_stratagems')
}

export async function getStratagem(name: string): Promise<Stratagem> {
  const find = (await getStratagems())
    .flatMap(it => it)
    .find(it => it.name === name)

  if (!find)
    throw new Error('Not found')

  return find
}
