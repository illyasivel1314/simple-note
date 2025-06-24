<template>
  <a-config-provider 
    :locale=zhCN
    :theme="{
      algorithm: theme.darkAlgorithm,
      token: {
        colorPrimary: '#ffffff',
        colorBgLayout: '#141414',
      },
    }"
  >
    <a-space class="global-style" :style="{ gap: '0px', width: '950px', height: '560px', opacity: opacitySystem }">
      <a-layout-sider
        width="250px"
        class="global-style-left"
      >
        <Note />
      </a-layout-sider>
      <a-layout-content
       class="global-style-right"
      >
        <Calendar/>
      </a-layout-content>
    </a-space>
  </a-config-provider>
</template>

<script setup lang="ts">
import { theme } from "ant-design-vue";
import Calendar from "/src/components/Calendar.vue";
import zhCN from 'ant-design-vue/es/locale/zh_CN';
import dayjs from 'dayjs';
import updateLocale from 'dayjs/plugin/updateLocale';
import 'dayjs/locale/zh-cn';
import Note from "/src/components/Note.vue";
import { computed, ref } from "vue";
import { platform } from '@tauri-apps/plugin-os';

dayjs.locale('zh-cn');
dayjs.extend(updateLocale);
dayjs.updateLocale('zh-cn', {
  weekStart: 0,
});

let system = ref(platform());
let opacitySystem = computed(() => {
  return system.value === 'windows' ? "0.8" : "1";
})


</script>
<style>

/* 取消用户对元素的选择 */
.global-style {
  -webkit-user-select: none; /* Safari */
  -ms-user-select: none; /* IE 10+ and Edge */
  user-select: none; /* Standard syntax */
}

.global-style-left {
  width: 250px;
}
.global-style-right {
  width: 700px;
  text-align: right;
}

:where(.css-dev-only-do-not-override-szjkbo).ant-radio-wrapper .ant-radio-checked .ant-radio-inner {
  background-color: rgba(0, 0, 0, 0)
}

</style>