<script setup>

</script>

<template>
  <div class="form-check">
    <input v-model="createDataURL" @change="textChange('clear')" class="form-check-input" type="checkbox" id="data-url-checkbox">
    <label class="form-check-label" for="data-url-checkbox">
      Create Base64-Data-URL
    </label>
  </div>
  <input v-model="dataURLMimeType" v-if="createDataURL" @keyup="textChange('clear')" class="form-control mt-1 mb-3" placeholder="Mimetype e.g. image/png" />

  <div class="row">
    <div class="col-sm-6">
      <label>Cleartext</label>
      <textarea v-model="clearText" @keyup="textChange('clear')" class="form-control" rows="10"></textarea>
    </div>
    <div class="col-sm-6">
      <label>Base64</label>
      <textarea v-model="base64Text" @keyup="textChange('base64')" class="form-control" rows="10"></textarea>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Base64Wrapper',
  data() {
    return {
      clearText: '',
      base64Text: '',
      createDataURL: false,
      dataURLMimeType: ''
    }
  },
  methods: {
    textChange(input) {
      if(input === 'clear') {
        try {
          let dataURI = '';
          if(this.createDataURL) {
            dataURI = 'data:';
            if(this.dataURLMimeType.length > 0) {
              dataURI += this.dataURLMimeType + ';';
            }
            else {
              dataURI += ',';
            }
          }

          this.base64Text = dataURI + window.btoa(this.clearText);
        }
        catch (e) {
          console.log('Failed to convert to base64. ' + e.message);
        }
      }
      else if(input === 'base64') {
        try {
          this.clearText = window.atob(this.base64Text);
        }
        catch (e) {
          console.log('Failed to convert to cleartext. ' + e.message);
        }
      }
    }
  }
}
</script>

<style scoped>

</style>