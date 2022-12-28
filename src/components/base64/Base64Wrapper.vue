<script setup>

</script>

<template>
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
      base64Text: ''
    }
  },
  methods: {
    textChange(input) {
      if(input === 'clear') {
        try {
          this.base64Text = window.btoa(this.clearText);
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