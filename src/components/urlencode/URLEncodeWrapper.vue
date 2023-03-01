<script setup>

</script>

<template>
  <div class="row">
    <div class="col-sm-6">
      <label class="d-block">
        Cleartext
        <a href="#" @click="copyText('clearText')" class="copy-button float-end">Copy</a>
      </label>
      <textarea v-model="clearText" @keyup="textChange('clear')" class="form-control" rows="10"></textarea>
    </div>
    <div class="col-sm-6">
      <label class="d-block">
        URL-Encode
        <a href="#" @click="copyText('urlText')" class="copy-button float-end">Copy</a>
      </label>
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
    copyText(type) {
      let copyText = type === 'clearText' ? this.clearText: this.urlText;
      navigator.clipboard.writeText(copyText);
    },
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
.copy-button {
  text-decoration: none;
}
</style>