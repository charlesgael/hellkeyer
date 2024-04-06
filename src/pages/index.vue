<script setup lang="ts">
import { getStratagems } from '~/api/stratagems'
import { bind, getBindings } from '~/api/bindings'

const { t } = useI18n()

const { state: stratagems } = useAsyncState(() => getStratagems(), [])
const { state: bound, execute: refreshBound } = useAsyncState(() => getBindings(), {})

const nextBinding = computed(() => {
  if (!bound.value.f5)
    return 'f5'
  if (!bound.value.f6)
    return 'f6'
  if (!bound.value.f7)
    return 'f7'
  if (!bound.value.f8)
    return 'f8'
  if (!bound.value.f9)
    return 'f9'
  if (!bound.value.f10)
    return 'f10'
  return null
})

function isBound(name: string) {
  return Object.entries(bound.value)
    .find(([,val]) => val === name)
    ?.[0]
}

async function stratagemClicked(name: string) {
  const bound_key = isBound(name)
  if (bound_key)
    await bind(bound_key, null)

  else
    if (nextBinding.value)
      bind(nextBinding.value, name)

  refreshBound()
}
</script>

<template lang="pug">
.layout
  .group(v-for="(group, index) in stratagems")
    h1 {{ t('categories.' + index) }}
    .items
      Stratagem(v-for="{name, sequence, icon} in group" :name="name" :sequence="sequence" :icon="icon" :bound="isBound(name)" @click="stratagemClicked(name)")

.shortcuts
  .shortcut
    h1 F1
    StratagemByName(name="reinforce")
  .shortcut
    h1 F2
    StratagemByName(name="resupply")
  .shortcut
    h1 F3
    StratagemByName(name="sos_beacon")
  .shortcut
    h1 F4
    ic:baseline-disabled-by-default.text-40px.text-red-600
  .shortcut(:class="nextBinding === 'f5' && 'highlight'")
    h1 F5
    StratagemByName(v-if="bound.f5" :name="bound.f5")
  .shortcut(:class="nextBinding === 'f6' && 'highlight'")
    h1 F6
    StratagemByName(v-if="bound.f6" :name="bound.f6")
  .shortcut(:class="nextBinding === 'f7' && 'highlight'")
    h1 F7
    StratagemByName(v-if="bound.f7" :name="bound.f7")
  .shortcut(:class="nextBinding === 'f8' && 'highlight'")
    h1 F8
    StratagemByName(v-if="bound.f8" :name="bound.f8")
  .shortcut(:class="nextBinding === 'f9' && 'highlight'")
    h1 F9
    StratagemByName(v-if="bound.f9" :name="bound.f9")
  .shortcut(:class="nextBinding === 'f10' && 'highlight'")
    h1 F10
    StratagemByName(v-if="bound.f10" :name="bound.f10")
  .shortcut
    h1 F11
    material-symbols:select-window.text-40px.text-blue-600
  .shortcut
    h1 F12
    ic:baseline-disabled-by-default.text-40px.text-red-600
</template>

<style lang="sass" scoped>
.layout
  @apply sm:columns-2 md:columns-3 lg:columns-4 xl:columns-5 p-2

  .group
    @apply flex flex-col
    break-inside: avoid

    h1
      @apply text-xl

.shortcuts
  @apply flex space-x-1

  :nth-child(4n):not([hidden])
    @apply mr-4

.shortcut
  @apply flex flex-col w-50px shadow border border-white

  &.highlight
    @apply border-dashed border-gray-300

  h1
    @apply bg-dark-700 text-white text-xs
</style>

<route lang="yaml">
meta:
  layout: home
</route>
