<template>
  <component :is="currentFrame" />
</template>

<script>
import HostWrapper from './host/HostWrapper.vue';
import Base64Wrapper from './base64/Base64Wrapper.vue';

export default {
  name: "MainFrame",
  components: {
    HostWrapper,
    Base64Wrapper
  },
  props: ['currentFrame']
}
</script>

<style scoped>

</style>