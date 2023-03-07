<template>
  <button class="btn btn-link float-end " type="button" data-bs-toggle="offcanvas" data-bs-target="#staticBackdrop" aria-controls="staticBackdrop">
    <i class="fa-solid fa-bars"></i>
  </button>

  <div class="offcanvas offcanvas-start" data-bs-backdrop="static" tabindex="-1" id="staticBackdrop" aria-labelledby="staticBackdropLabel">
    <div class="offcanvas-header">
      <h5 class="offcanvas-title" id="staticBackdropLabel">Menu</h5>
      <button type="button" class="btn-close" id="btn-close-menu" data-bs-dismiss="offcanvas" aria-label="Close"></button>
    </div>
    <div class="offcanvas-body">
      <h5>Windows</h5>
      <ul class="navbar-nav justify-content-end flex-grow-1 pe-3">
        <li class="nav-item">
          <a class="nav-link" href="#" @click="switchMenu('HostWrapper')">Edit Host-File</a>
        </li>
        <li class="nav-item">
          <a class="nav-link" href="#" @click="switchMenu('PathWrapper')">Edit Path-Variables</a>
        </li>
      </ul>
      <h5 class="mt-2">Other</h5>
      <ul class="navbar-nav justify-content-end flex-grow-1 pe-3">
        <li class="nav-item">
          <a class="nav-link" href="#" @click="switchMenu('Base64Wrapper')">Base64 Encode/Decode</a>
        </li>
        <li class="nav-item">
          <a class="nav-link" href="#" @click="switchMenu('URLEncodeWrapper')">URL Encode/Decode</a>
        </li>
      </ul>

      <div class="position-absolute bottom-0" id="copyright-wrapper">
        <p>
          Created by David R. - Version 0.1.0
          <a @click="openURL('https://github.com/Xcreen/windows-tool-set')" target="_blank" class="float-end" style="cursor: pointer;">
            <i class="fa-brands fa-github"></i>
          </a>
        </p>
      </div>
    </div>
  </div>
</template>

<script>
import { open } from '@tauri-apps/api/shell';

export default {
  name: "Navigation",
  emits: ['switchEvent'],
  methods: {
    switchMenu(menuItem) {
      this.$emit('switchEvent', menuItem);
    },
    closeMenu() {
      document.getElementById('btn-close-menu').click();
    },
    openURL(url) {
      event.preventDefault();
      open(url);
    }
  }
}
</script>

<style scoped>
#copyright-wrapper {
  width: calc(100% - 30px);
}
</style>