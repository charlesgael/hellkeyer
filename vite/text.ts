import { readFileSync } from 'node:fs'
import { createFilter, createLogger } from 'vite'
import type { Plugin } from 'vite'

type Filter = ReadonlyArray<string | RegExp> | string | RegExp
interface Options {
  include: Filter
  exclude?: Filter
}

export function text(opts: Options): Plugin {
  const filter = createFilter(opts.include, opts.exclude)
  const log = createLogger()

  return {
    name: 'text',

    transform(_, id) {
      if (filter(id)) {
        const code = readFileSync(id).toString('utf-8')

        return {
          map: { mappings: '' },
          code: `export default "${code}";`,
        }
      }
    },
  }
}
