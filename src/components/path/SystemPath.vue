<template>
  <button @click="addPath" class="btn btn-primary mt-2">
    <i class="fa-solid fa-plus"></i> Add Path
  </button>

  <div class="row mb-3 mt-3" v-for="(variable, index) in pathData">
    <div class="col-sm-9">
      <input type="text" class="form-control" v-model="inputEditPaths[index]">
    </div>
    <div class="col-sm-3">
      <button @click="move(index, 'up')" class="btn btn-primary">
        <i class="fa-solid fa-chevron-up"></i>
      </button>
      <button @click="move(index, 'down')" class="btn btn-primary ms-2">
        <i class="fa-solid fa-chevron-down"></i>
      </button>
      <button @click="deleteEntry(index)" class="btn btn-danger ms-2">
        <i class="fa-solid fa-trash"></i>
      </button>
    </div>
  </div>

  <button @click="save" class="btn btn-success w-100 mt-2">
    <i class="fa-solid fa-check"></i> Save
  </button>
</template>

<script>
import {invoke} from '@tauri-apps/api/tauri';

export default {
  name: 'SystemPath',

  data() {
    return {
      pathData: [],
      inputEditPaths: [],
    }
  },

  async mounted() {
    const systemVariables = await invoke("get_system_variables", {});
    systemVariables.forEach((pathData, index) => {
      let data = {};
      data.path = pathData;
      this.pathData.push(data);
      this.inputEditPaths[index] = pathData;
    });
  },

  methods: {
    addPath() {
      let data = {};
      data.path = '';
      this.pathData.push(data);
      this.inputEditPaths.push('');
    },
    move(index, direction) {
      if (direction === 'up' && index === 0) return;
      if (direction === 'down' && (index + 1) === this.pathData.length) return;

      let newPosition = 0;
      if (direction === 'up') newPosition = index - 1;
      if (direction === 'down') newPosition = index + 1;

      const pathData = this.pathData[index];
      const inputData = this.inputEditPaths[index];

      this.pathData.splice(index, 1);
      this.pathData.splice(newPosition, 0, pathData);

      this.inputEditPaths.splice(index, 1);
      this.inputEditPaths.splice(newPosition, 0, inputData);
    },
    deleteEntry(index) {
      this.pathData.splice(index, 1);
      this.inputEditPaths.splice(index, 1);
    },
    async save() {
      const systemPath = this.inputEditPaths.join(";").trim();
      let saveResult = await invoke("save_system_variables", { systemPath });
      if(saveResult) {
        this.emitter.emit('showToastEvent', { type: 'success', message: 'Saved system-path!'});
      }
      else {
        this.emitter.emit('showToastEvent', { type: 'error', message: 'Failed to save system-path!'});
      }
    }
  }
}
</script>

<style scoped>

</style>