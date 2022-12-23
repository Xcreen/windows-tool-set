<script setup>
import HostWrapper from './host/HostWrapper.vue';
</script>

<template>
  <HostWrapper v-if="currentFrame === 'HostWrapper'" />
</template>

<script>
export default {
  name: "MainFrame",
  data() {
    return {
      currentFrame: 'HostWrapper'
    }
  },
}
</script>

<style scoped>

</style>