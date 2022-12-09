<script setup>
import HostFile from './components/HostFile.vue';
</script>

<template>
  <div id="main-container">
    <ul class="nav nav-tabs" role="tablist">
      <li class="nav-item" role="presentation">
        <button class="nav-link active" id="host-tab" data-bs-toggle="tab" data-bs-target="#host-tab-pane" type="button" role="tab" aria-selected="true">
          Host
        </button>
      </li>
      <li class="nav-item" role="presentation">
        <button class="nav-link" id="host-entries-tab" data-bs-toggle="tab" data-bs-target="#host-entries-tab-pane" type="button" role="tab" aria-selected="false">
          Host-Entries
        </button>
      </li>
      <li class="nav-item" role="presentation">
        <button class="nav-link" id="environment-variables-tab" data-bs-toggle="tab" data-bs-target="#environment-variables-tab-pane" type="button" role="tab" aria-selected="false">
          Environment-Variables
        </button>
      </li>
    </ul>

    <div class="tab-content">
      <div class="tab-pane show active" id="host-tab-pane" role="tabpanel" aria-labelledby="host-tab" tabindex="0">
        <HostFile />
      </div>
      <div class="tab-pane " id="host-entries-tab-pane" role="tabpanel" aria-labelledby="host-entries-tab" tabindex="0">Coming soon ...</div>
      <div class="tab-pane " id="environment-variables-tab-pane" role="tabpanel" aria-labelledby="environment-variables-tab" tabindex="0">Coming soon ...</div>
    </div>


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

<style scoped>
  #main-container {
    padding: 20px;
    min-height: 100vh;
  }
</style>
