<template>
  <div class="splash">
    <img src="/loading.gif" :style="{ width: '350px', height: '350px' }" alt="数据加载中，请稍候..." draggable="false">
  </div>
</template>

<script setup lang="ts">

</script>

<style scoped>
.splash {
  width: 600px;
  height: 350px;
  background-color: antiquewhite;
  display: flex;
  justify-content: center;
  align-items: center;
  
  -webkit-user-select: none; /* Safari */
  -moz-user-select: none; /* Firefox */
  -ms-user-select: none; /* IE10+ */
  user-select: none; /* 标准语法 */
}
</style>