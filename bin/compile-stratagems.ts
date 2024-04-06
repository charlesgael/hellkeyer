import { copyFileSync, existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, renameSync, rmdirSync, unlinkSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'

const rootPath = join(__dirname, '..')
const stratagemsPath = join(rootPath, 'data', 'stratagems')
const publicPath = join(rootPath, 'public')
const publicStratagems = join(publicPath, 'stratagems')

const entries = readdirSync(stratagemsPath)

const groups = [
  [
    'sos_beacon',
    'resupply',
    'reinforce',
  ],
  [
    'ballistic_shield_backpack',
    'guard_dog_rover',
    'guard_dog',
    'jump_pack',
    'shield_generator_pack',
    'supply_pack',
  ],
  [
    'anti_materiel_rifle',
    'arc_thrower',
    'autocannon',
    'expendable_anti_tank',
    'flamethrower',
    'grenade_launcher',
    'heavy_machine_gun',
    'laser_cannon',
    'machine_gun',
    'quasar_canon',
    'railgun',
    'recoilless_rifle',
    'spear',
    'stalwart',
  ],
  [
    'anti_personnel_minefield',
    'autocannon_sentry',
    'ems_mortar_sentry',
    'gatling_sentry',
    'hmg_emplacement',
    'incendiary_mines',
    'machine_gun_sentry',
    'mortar_sentry',
    'rocket_sentry',
    'shield_generator_relay',
    'tesla_tower',
  ],
  [
    'orbital_120mm_he_barrage',
    'orbital_380mm_he_barrage',
    'orbital_airburst_strike',
    'orbital_ems_strike',
    'orbital_gas_strike',
    'orbital_gatling_barrage',
    'orbital_laser',
    'orbital_precision_strike',
    'orbital_railcannon_strike',
    'orbital_smoke_strike',
    'orbital_walking_barrage',
  ],
  [
    'eagle_110mm_rochet_pods',
    'eagle_500kg_bomb',
    'eagle_airstrike',
    'eagle_cluster_bomb',
    'eagle_napalm_airstrike',
    'eagle_smoke_strike',
    'eagle_strafing_run',
  ],
  [
    'patriot_exosuit',
  ],
]

// Clean public stratagems
if (existsSync(publicStratagems)) {
  console.log('> Removing existing \'public/stratagems\' folder')
  rmdirSync(publicStratagems, { recursive: true, force: true } as any)
}

// Verify data
console.log('> Data validation')
for (const group of groups) {
  for (const stratagem of group) {
    if (!existsSync(join(stratagemsPath, stratagem))) {
      console.log(`Stratagem '${stratagem}' not found`)
      continue
    }

    if (!existsSync(join(stratagemsPath, stratagem, 'sequence.txt'))) { console.log(`No sequence file for '${stratagem}'`) }
    else {
      const sequence = readFileSync(join(stratagemsPath, stratagem, 'sequence.txt')).toString()
      if (!sequence)
        console.log(`No sequence for '${stratagem}'`)
    }

    if (!existsSync(join(stratagemsPath, stratagem, 'icon.png')))
      console.log(`No icon for '${stratagem}'`)
  }
}

// Computing loose entries
console.log('> Computing loose entries')
const allEntries = groups.flatMap(it => it)
const looseEntries = entries.filter(it => !allEntries.includes(it))
if (looseEntries.length)
  console.log('looseEntries =', looseEntries)
else
  console.log('None found')

// Creation of folders
console.log('> Creating \'public/stratagems\' folder')
mkdirSync(publicStratagems)
for (const groupId in groups) {
  const start = Number.parseInt(groupId) * 1000
  const group = groups[groupId]

  for (const stratId in group) {
    const stratagem = group[stratId]
    const fullId = (`0000${start + Number.parseInt(stratId)}`).slice(-4)
    const origPath = join(stratagemsPath, stratagem)
    const destPath = join(publicStratagems, String(fullId))

    mkdirSync(join(destPath))
    writeFileSync(join(destPath, 'name.txt'), stratagem)
    if (existsSync(join(origPath, 'sequence.txt')))
      copyFileSync(join(origPath, 'sequence.txt'), join(destPath, 'sequence.txt'))
    if (existsSync(join(origPath, 'icon.png')))
      copyFileSync(join(origPath, 'icon.png'), join(destPath, 'icon.png'))
  }
}

if (looseEntries.length) {
  console.log('> Creating loose stratagems')
  const start = groups.length * 1000

  for (const stratId in looseEntries) {
    const stratagem = looseEntries[stratId]
    const fullId = start + Number.parseInt(stratId)
    const origPath = join(stratagemsPath, stratagem)
    const destPath = join(publicStratagems, String(fullId))

    mkdirSync(join(destPath))
    writeFileSync(join(origPath, 'name.txt'), stratagem)
    if (existsSync(join(origPath, 'sequence.txt')))
      copyFileSync(join(origPath, 'sequence.txt'), join(destPath, 'sequence.txt'))
    if (existsSync(join(origPath, 'icon.png')))
      copyFileSync(join(origPath, 'icon.png'), join(destPath, 'icon.png'))
  }
}
