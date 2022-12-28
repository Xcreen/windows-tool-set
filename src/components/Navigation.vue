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
<!--        <li class="nav-item">
          <a class="nav-link" href="#">Edit Environment-Variables</a>
        </li>-->
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
    </div>
  </div>
</template>

<script>
export default {
  name: "Navigation",
  emits: ['switchEvent'],
  methods: {
    switchMenu(menuItem) {
      this.$emit('switchEvent', menuItem);
    },
    closeMenu() {
      document.getElementById('btn-close-menu').click();
    }
  }
}
</script>

<style scoped>

</style>