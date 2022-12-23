<template>
  <div :class="[{ show: showToast }, toastTypeClass]" class="toast align-items-center border-0" role="alert" aria-live="assertive" aria-atomic="true">
    <div class="d-flex">
      <div class="toast-body">{{ messageBody }}</div>
      <button @click="showToast = false" class="btn-close btn-close-white me-2 m-auto"></button>
    </div>
  </div>
</template>

<script>
const defaultMessage = {
  type: 'success',
  message: '',
  timeout: 5000
}

let toastTimeout;

export default {
  name: 'Notification',
  data() {
    return {
      messageBody: '',
      toastTypeClass: 'text-bg-success',
      showToast: false,
      showErrorToast: false
    }
  },
  mounted() {
    this.emitter.on('showToastEvent', (eventData) => {
      let message = defaultMessage;
      Object.assign(message, eventData);
      this.messageBody = message.message;
      this.showToast = true;
      if(message.type === 'success') {
        this.toastTypeClass =  'text-bg-success';
      }
      else {
        this.toastTypeClass =  'text-bg-danger';
      }
      clearTimeout(toastTimeout);
      toastTimeout = setTimeout(() => {
        this.showToast = false;
      }, message.timeout);
    });
  }
}
</script>

<style scoped>

</style>