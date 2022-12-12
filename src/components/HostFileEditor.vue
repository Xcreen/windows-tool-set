<script>
import { invoke } from "@tauri-apps/api/tauri";

export default {

  data() {
    return {
      inputAddIP: '',
      inputAddDomain: '',
      inputAddDescription: '',
      cleanLines: []
    }
  },

  async mounted() {
    const hostFileLines = await invoke("read_host_file_lines", {});
    this.cleanLines = [];
    hostFileLines.forEach((line, index) => {
      const tmpLine = line.trim();
      if(tmpLine.length > 0 && !tmpLine.startsWith('#')) {
        const tmpData = {};
        tmpData.originalLine = line;
        tmpData.cleanLine = tmpLine;
        tmpData.lineNumber = index + 1;
        this.cleanLines.push(tmpData);
      }
    });
  },

  methods: {
    addHostEntry() {
      let ip = '127.0.0.1';
      if(this.inputAddIP.trim().length > 0) {
        ip = this.inputAddIP.trim();
      }
      if(this.inputAddDomain.trim().length === 0) {
        return;
      }
      let hostLine = ip + ' ' + this.inputAddDomain.trim();
      if(this.inputAddDescription.trim().length > 0) {
        hostLine += ' # ' + this.inputAddDescription.trim();
      }

      

      document.querySelector('#add-entry-modal .btn-close').click();

      const successToast = document.getElementById('success-toast');
      successToast.querySelector('.toast-body').innerText = 'Added Host-Entry!';
      successToast.classList.add('show');
    }
  }
}
</script>

<template>
  <div class="host-file-entry-wrapper">
    <button class="btn btn-primary mt-2" data-bs-toggle="modal" data-bs-target="#add-entry-modal">Add Host-Entry</button>


  </div>

  <!-- Add-Entry-Modal -->
  <div class="modal fade" id="add-entry-modal" tabindex="-1" aria-hidden="true">
    <div class="modal-dialog modal-xl">
      <div class="modal-content">
        <div class="modal-header">
          <h1 class="modal-title fs-5">Add Host-Entry</h1>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>
        <div class="modal-body">
          <div class="row">
            <div class="col-sm-6">
              <label for="add-ip-input" class="form-label">IP</label>
              <input v-model="inputAddIP" class="form-control" list="ipDataList" id="add-ip-input">
              <datalist id="ipDataList">
                <option value="127.0.0.1" />
              </datalist>
            </div>
            <div class="col-sm-6">
              <label for="add-domain-input" class="form-label">Domain</label>
              <input v-model="inputAddDomain" class="form-control" :class="{ 'is-invalid' : inputAddDomain.length === 0 }" id="add-domain-input">
            </div>
            <div class="col-sm-12">
              <label for="add-description-input" class="form-label">Description</label>
              <input v-model="inputAddDescription" class="form-control" id="add-description-input">
            </div>
          </div>
        </div>
        <div class="modal-footer">
          <button @click="addHostEntry" class="btn btn-primary">Add entry</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>

</style>