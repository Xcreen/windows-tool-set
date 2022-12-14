<script>
import { invoke } from "@tauri-apps/api/tauri";

export default {

  data() {
    return {
      inputAddIP: '',
      inputAddDomain: '',
      inputAddDescription: '',
      inputEditIPs: [],
      inputEditDomain: [],
      inputEditDescription: [],
      cleanLines: [],
      deleteLineNo: -1
    }
  },

  async mounted() {
    this.emitter.on('refreshHostEntries', () => {
      this.readHostLines();
    });

    await this.readHostLines();
  },

  methods: {
    async refreshData() {
      this.emitter.emit('refreshHostDataEvent');
    },
    async readHostLines() {
      const hostFileLines = await invoke("read_host_file_lines", {});
      this.cleanLines = [];
      hostFileLines.forEach((line, index) => {
        const tmpLine = line.trim();
        if(tmpLine.length > 0 && !tmpLine.startsWith('#')) {
          const lineArr = tmpLine.split(" ");
          const tmpData = {};
          tmpData.originalLine = line;
          tmpData.cleanLine = tmpLine;
          tmpData.lineNumber = index + 1;
          tmpData.ip = '';
          tmpData.domain = '';
          tmpData.description = '';
          if(lineArr.length > 0) {
            tmpData.ip = lineArr[0];
            if(lineArr.length > 1) {
              tmpData.domain = lineArr[1];
            }
            if(lineArr.length > 3) {
              if(lineArr[2] === '#') {
                let tmpDescription = '';
                for(let i = 3; i < lineArr.length; i++) {
                  tmpDescription += lineArr[i] + ' ';
                }
                tmpData.description = tmpDescription;
              }
            }
          }
          this.cleanLines.push(tmpData);
        }
      });

      this.cleanLines.forEach((lineData, index) => {
        this.inputEditIPs[index] = lineData.ip;
        this.inputEditDomain[index] = lineData.domain;
        this.inputEditDescription[index] = lineData.description;
      });
    },
    async addHostEntry() {
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

      let saveResult = await invoke("append_entry_to_host_file", { newLine: hostLine });

      document.querySelector('#add-entry-modal .btn-close').click();

      if(saveResult) {
        this.emitter.emit('showToastEvent', { type: 'success', message: 'Added Host-Entry!'});
        await this.refreshData();
      }
      else {
        this.emitter.emit('showToastEvent', { type: 'error', message: 'Failed to save file!'});
      }
    },
    async editHostEntry(lineNumber, inputIndex) {
      let ip = '127.0.0.1';
      if(this.inputEditIPs[inputIndex].trim().length > 0) {
        ip = this.inputEditIPs[inputIndex].trim();
      }
      if(this.inputEditDomain[inputIndex].trim().length === 0) {
        return;
      }
      let hostLine = ip + ' ' + this.inputEditDomain[inputIndex].trim();
      if(this.inputEditDescription[inputIndex].trim().length > 0) {
        hostLine += ' # ' + this.inputEditDescription[inputIndex].trim();
      }

      let saveResult = await invoke('edit_host_line', { lineNo: lineNumber, newHostEntry: hostLine });
      if(saveResult) {
        this.emitter.emit('showToastEvent', { type: 'success', message: 'Updated Host-Entry!'});
        await this.refreshData();
      }
      else {
        this.emitter.emit('showToastEvent', { type: 'error', message: 'Failed to update entry!'});
      }
    },
    async confirmDeleteHostEntry(lineNumber) {
      this.deleteLineNo = lineNumber;
      document.getElementById('open-delete-modal-button').click();
    },
    async deleteHostEntry() {
      let saveResult = await invoke("delete_entry_from_host_file", { deleteLine: this.deleteLineNo });

      if(saveResult) {
        this.emitter.emit('showToastEvent', { type: 'success', message: 'Removed Host-Entry!'});
      }
      else {
        this.emitter.emit('showToastEvent', { type: 'error', message: 'Failed to remove entry!'});
      }

      await this.refreshData();
      document.querySelector('#delete-entry-modal .btn-close').click();
    }
  }
}
</script>

<template>
  <button id="open-delete-modal-button" class="d-none" data-bs-toggle="modal" data-bs-target="#delete-entry-modal">Open Delete-Modal</button>

  <div class="host-file-entry-wrapper">
    <button class="btn btn-primary mt-2" data-bs-toggle="modal" data-bs-target="#add-entry-modal">
      <i class="fa-solid fa-plus"></i> Add Host-Entry
    </button>
    <div class="row mt-3">
      <div class="col-sm-3">
        <label>IP</label>
      </div>
      <div class="col-sm-3">
        <label>Domain</label>
      </div>
      <div class="col-sm-3">
        <label>Description</label>
      </div>
    </div>

    <div class="row mb-3" v-for="(line, index) in cleanLines">
      <div class="col-sm-3">
        <input type="text" class="form-control" v-model="inputEditIPs[index]">
      </div>
      <div class="col-sm-3">
        <input type="text" class="form-control" v-model="inputEditDomain[index]">
      </div>
      <div class="col-sm-3">
        <input type="text" class="form-control" v-model="inputEditDescription[index]">
      </div>
      <div class="col-sm-3">
        <button @click="editHostEntry(line.lineNumber, index)" class="btn btn-success">
          <i class="fa-solid fa-check"></i>
        </button>
        <button @click="confirmDeleteHostEntry(line.lineNumber)" class="btn btn-danger ms-2">
          <i class="fa-solid fa-trash"></i>
        </button>
      </div>
    </div>
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
          <button class="btn btn-outline-secondary" data-bs-dismiss="modal">Close</button>
          <button @click="addHostEntry" class="btn btn-primary">
            <i class="fa-solid fa-plus"></i> Add entry
          </button>
        </div>
      </div>
    </div>
  </div>

  <!-- Delete-Entry-Modal -->
  <div class="modal fade" id="delete-entry-modal" tabindex="-1" aria-hidden="true">
    <div class="modal-dialog modal-xl">
      <div class="modal-content">
        <div class="modal-header">
          <h1 class="modal-title fs-5">Delete Host-Entry</h1>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>
        <div class="modal-body">
          You really want to delete this entry?
        </div>
        <div class="modal-footer">
          <button class="btn btn-outline-secondary" data-bs-dismiss="modal">Close</button>
          <button @click="deleteHostEntry" class="btn btn-danger">
            <i class="fa-solid fa-trash"></i> Delete entry
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>

</style>