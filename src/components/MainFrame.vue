<template>
  <component :is="currentFrame" />
</template>

<script>
import HostWrapper from './host/HostWrapper.vue';
import Base64Wrapper from './base64/Base64Wrapper.vue';
import URLEncodeWrapper from './urlencode/URLEncodeWrapper.vue';

export default {
  name: "MainFrame",
  components: {
    HostWrapper,
    Base64Wrapper,
    URLEncodeWrapper
  },
  props: ['currentFrame']
}
</script>

<style scoped>

</style>