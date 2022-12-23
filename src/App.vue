<script setup>
import Navigation from './components/Navigation.vue';
import MainFrame from './components/MainFrame.vue';
</script>

<template>
  <Navigation ref="navigationRef" @switch-event="switchComponent" />

  <div id="main-container">
    <MainFrame :currentFrame="currentFrame" />

    <div class="toast-container position-fixed bottom-0 end-0 p-3">
      <div id="success-toast" class="toast align-items-center text-bg-success border-0" role="alert" aria-live="assertive" aria-atomic="true">
        <div class="d-flex">
          <div class="toast-body"></div>
          <button type="button" class="btn-close btn-close-white me-2 m-auto" data-bs-dismiss="toast" aria-label="Close"></button>
        </div>
      </div>
      <div id="error-toast" class="toast align-items-center text-bg-danger border-0" role="alert" aria-live="assertive" aria-atomic="true">
        <div class="d-flex">
          <div class="toast-body"></div>
          <button type="button" class="btn-close btn-close-white me-2 m-auto" data-bs-dismiss="toast" aria-label="Close"></button>
        </div>
      </div>
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
