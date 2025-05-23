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
    <a-space :style="{ gap: '0px', width: '950px', height: '560px' }">
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

dayjs.locale('zh-cn');
dayjs.extend(updateLocale);
dayjs.updateLocale('zh-cn', {
  weekStart: 0,
});


</script>
<style>

.global-style-left {
  width: 250px;
}
.global-style-right {
  width: 700px;
  text-align: right;
}

</style>