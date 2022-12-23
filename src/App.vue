<script setup>
import Navigation from './components/Navigation.vue';
import MainFrame from './components/MainFrame.vue';
import Notification from './components/Notification.vue';
</script>

<template>
  <Navigation ref="navigationRef" @switch-event="switchComponent" />

  <div id="main-container">
    <MainFrame :currentFrame="currentFrame" />

    <div class="toast-container position-fixed bottom-0 end-0 p-3">
      <Notification />
    </div>
  </div>
</template>

<script>
import {ref} from 'vue';

const navigationRef = ref();

export default {
  name: 'App',
  data() {
    return {
      currentFrame: 'HostWrapper'
    }
  },
  methods: {
    switchComponent(menuItem) {
      this.currentFrame = menuItem;
      navigationRef.value.closeMenu();
    }
  }
}
</script>

<style scoped>
  #main-container {
    padding: 20px;
    min-height: 100vh;
  }
</style>
