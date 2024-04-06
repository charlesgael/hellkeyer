<script lang="ts" setup>
import cn from 'classnames'

const $p = defineProps<{
  name: string
  sequence: string
  icon: string
  expand?: boolean
  bound?: boolean
}>()

const { t } = useI18n()

const LETTERS_MAP = {
  U: 'up',
  D: 'down',
  L: 'left',
  R: 'right',
}

const arrows = computed(() => {
  const letters = $p.sequence?.toUpperCase().split('') || []
  return letters
    .filter(l => (LETTERS_MAP as any)[l])
    .map(l => (LETTERS_MAP as any)[l])
})
</script>

<template lang="pug">
.wrapper(:title="expand || t(`stratagem.${name}`)")
  .stratagem(:class="cn({ expand, bound})" :data-popover-target="`popover-stratagem-${name}`")
    .icon
      img(:src="icon" width="40" height="40"/* onerror="this.onerror = null; this.src = 'assets/noicon.png';"*/)
    .body(v-if="expand")
      .name {{ t("stratagem." + name) }}
      .sequence
        div(v-for="arrow in arrows" :class="'icon-arrow-' + arrow")
</template>

<style lang="sass" scoped>
.stratagem
  // background: #030f15http://localhost:3333/
  @apply p-2 inline-flex space-x-2 items-center
  @apply transition-all
  @apply shadow-sm hover:shadow-md shadow-dark-500

  &.expand
    @apply flex

  &.bound
    @apply bg-gray-500

  .icon
    @apply rounded rounded-md overflow-hidden

.sequence
  @apply flex space-x-1

.body
  @apply flex flex-col min-w-0

.name
  @apply overflow-hidden overflow-ellipsis text-nowrap

.wrapper
  @apply inline-block relative
</style>
