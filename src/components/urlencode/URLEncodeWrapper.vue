<script setup>

</script>

<template>
  <div class="row">
    <div class="col-sm-6">
      <label>Cleartext</label>
      <textarea v-model="clearText" @keyup="textChange('clear')" class="form-control" rows="10"></textarea>
    </div>
    <div class="col-sm-6">
      <label>URL-Encode</label>
      <textarea v-model="urlText" @keyup="textChange('url')" class="form-control" rows="10"></textarea>
    </div>
  </div>
</template>

<script>
export default {
  name: 'URLEncodeWrapper',
  data() {
    return {
      clearText: '',
      urlText: ''
    }
  },
  methods: {
    textChange(input) {
      if(input === 'clear') {
        try {
          this.urlText = window.encodeURIComponent(this.clearText);
        }
        catch (e) {
          console.log('Failed to encode. ' + e.message);
        }
      }
      else if(input === 'url') {
        try {
          this.clearText = window.decodeURIComponent(this.urlText);
        }
        catch (e) {
          console.log('Failed to decode. ' + e.message);
        }
      }
    }
  }
}
</script>

<style scoped>

</style>