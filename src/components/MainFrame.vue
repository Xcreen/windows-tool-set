<template>
  <component :is="currentFrame" />
</template>

<script>
import HostWrapper from './host/HostWrapper.vue';
import Base64Wrapper from './base64/Base64Wrapper.vue';
import URLEncodeWrapper from './urlencode/URLEncodeWrapper.vue';
import PathWrapper from './path/PathWrapper.vue';

export default {
  name: "MainFrame",
  components: {
    HostWrapper,
    Base64Wrapper,
    URLEncodeWrapper,
    PathWrapper
  },
  props: ['currentFrame']
}
</script>

<style scoped>

</style>