<template>
  <component :is="currentFrame" />
</template>

<script>
import HostWrapper from './host/HostWrapper.vue';

export default {
  name: "MainFrame",
  components: {
    HostWrapper,
  },
  props: ['currentFrame']
}
</script>

<style scoped>

</style>