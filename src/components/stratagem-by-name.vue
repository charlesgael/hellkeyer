<script lang="ts" setup>
import { getStratagem } from '~/api/stratagems'

const $p = defineProps<{
  name: string
}>()

const { state: stratagem } = useAsyncState(() => getStratagem($p.name), undefined, {})
</script>

<template lang="pug">
template(v-if="stratagem")
  img(:src="stratagem.icon")
</template>
