<script>
import { invoke } from "@tauri-apps/api/tauri";

export default {

  data() {
    return {
      hostContent: ''
    }
  },

  async mounted() {
    this.hostContent = await invoke("read_host_file", {});
  },

  methods: {
    async save() {
      let saveResult = await invoke("save_host_file", { hostContent: this.hostContent });
      if(saveResult) {
        this.emitter.emit('showToastEvent', { type: 'success', message: 'Saved file!'});
      }
      else {
        this.emitter.emit('showToastEvent', { type: 'error', message: 'Failed to save file!'});
      }
    }
  }
}
</script>

<template>
  <div class="host-file-wrapper">
    <textarea class="form-control mt-2" v-model="hostContent"></textarea>
    <button class="btn btn-success w-100 mt-2" v-bind:class="{ disabled: (hostContent.length === 0) }" @click="save">
      <i class="fa-solid fa-check"></i> Save
    </button>
  </div>
</template>

<style scoped>
  .host-file-wrapper {
    height: 100%;
  }

  textarea {
    min-height: 450px;
  }
</style>