<script setup>
import HostFile from './HostFile.vue';
import HostFileEditor from './HostFileEditor.vue';
</script>

<template>
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
  </ul>

  <div class="tab-content">
    <div class="tab-pane show active" id="host-tab-pane" role="tabpanel" aria-labelledby="host-tab" tabindex="0">
      <HostFile />
    </div>
    <div class="tab-pane " id="host-entries-tab-pane" role="tabpanel" aria-labelledby="host-entries-tab" tabindex="0">
      <HostFileEditor />
    </div>
  </div>
</template>

<script>
export default {
  name: 'HostWrapper',
  mounted() {
    this.emitter.on('refreshHostDataEvent', () => {
      this.emitter.emit('refreshHostEntries');
      this.emitter.emit('refreshHostFile');
    });
  }
}
</script>

<style scoped>

</style>