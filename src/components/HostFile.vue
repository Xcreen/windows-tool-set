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
        let successToast = document.getElementById('success-toast');
        successToast.querySelector('.toast-body').innerText = 'Saved file!';
        successToast.classList.add('show');
      }
      else {
        let errorToast = document.getElementById('error-toast');
        errorToast.querySelector('.toast-body').innerText = 'Failed to save file!';
        errorToast.classList.add('show');
      }
    }
  }
}
</script>

<template>
  <div class="host-file-wrapper">
    <textarea class="form-control mt-2" v-model="hostContent"></textarea>
    <button class="btn btn-success w-100 mt-2" v-bind:class="{ disabled: (hostContent.length === 0) }" @click="save">Save</button>
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