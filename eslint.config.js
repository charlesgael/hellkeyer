// @ts-check
import antfu from '@antfu/eslint-config'
// @ts-expect-error no typing existing for this one
import vuePug from 'eslint-plugin-vue-pug'

export default antfu(
  {
    plugins: {
      'vue-pug': vuePug,
    },
    unocss: true,
    formatters: true,
    rules: {
      'unused-imports/no-unused-vars': 'off',

      'vue/attributes-order': 'warn',
      'vue/component-tags-order': 'warn',
      'vue/no-lone-template': 'warn',
      'vue/no-multiple-slot-args': 'warn',
      'vue/no-v-html': 'warn',
      'vue/order-in-components': 'warn',
      'vue/this-in-template': 'warn',
      'vue-pug/no-parsing-error': 'warn',
      'vue-pug/no-pug-control-flow': 'warn',
    },
  },
)
