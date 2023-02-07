<script setup>
import UserPath from './UserPath.vue';
import SystemPath from './SystemPath.vue';
</script>

<template>
  <ul class="nav nav-tabs" role="tablist">
    <li class="nav-item" role="presentation">
      <button class="nav-link active" id="host-tab" data-bs-toggle="tab" data-bs-target="#user-variables-tab-pane" type="button" role="tab" aria-selected="true">
        User-Path
      </button>
    </li>
    <li class="nav-item" role="presentation">
      <button class="nav-link" id="host-entries-tab" data-bs-toggle="tab" data-bs-target="#system-variables-tab-pane" type="button" role="tab" aria-selected="false">
        System-Path
      </button>
    </li>
  </ul>

  <div class="tab-content">
    <div class="tab-pane show active" id="user-variables-tab-pane" role="tabpanel" tabindex="0">
      <UserPath />
    </div>
    <div class="tab-pane " id="system-variables-tab-pane" role="tabpanel" tabindex="0">
      <SystemPath />
    </div>
  </div>
</template>

<script>
export default {
  name: 'PathWrapper'
}
</script>

<style scoped>

</style>