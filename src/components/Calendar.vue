<template>
  <a-calendar v-model:value="current_date" :style="{ height: '560px' }">
    <template #headerRender>
      <div style="padding-left: 10px; padding-top: 10px;">
        <a-row type="flex" :style="{ height: '40px' }" >
          <!-- 左右切换月份 -->
          <a-col>
            <a-space :size="10" class="out-lined-style">
              <a-button type="link" size="'small'" :icon="h(LeftOutlined)" @click="lastMonthChange" />
              <br>
              <a-button type="link" :icon="h(RightOutlined)" @click="nextMonthChange" />
            </a-space>
          </a-col>

          <!-- 月份选择器 -->
          <a-col :style="{ margin: '-2px 0 0 0' }">
            <a-date-picker
            picker="month" :value="current_date"
            :size="'large'"
            :allowClear="false" 
            :bordered="false" 
            :format="'YYYY年MM月'"
            :inputReadOnly="true"
            @change="dataPickerChange"
            :locale="locale"
            >
              <template #suffixIcon>
                <DownOutlined :style="{color: '#ffffff', fontSize: '10px'}" />
              </template>
            </a-date-picker>
          </a-col>

        </a-row>
      </div>
    </template>
    <template #dateCellRender="{ current }">
      
      <div class="calendar-content" :style="getCalendarContent(current)?.within_month == true ? 'opacity: 1' : 'opacity: 0.2'" >
        <div class="calendar-content-solar">{{ getCalendarContent(current)?.solar_calendar }}</div>
        <div class="calendar-content-rest" v-show="getCalendarContent(current)?.rest_day_valid !== null">
          <div class="calendar-content-rest-work" v-if="getCalendarContent(current)?.rest_day_valid == 0">
            班
          </div>
          <div class="calendar-content-rest-happy" v-else-if="getCalendarContent(current)?.rest_day_valid == 1">
            休
          </div>
        </div>
        <div class="calendar-content-lunar">{{ getCalendarContent(current)?.lunar_calendar }}</div>
      </div>
        
      <!-- <div v-if="getCalendarContent(current)?.within_month == true">
        <div class="calendar-content-text" style="color: orange;" v-if="getCalendarContent(current)?.todo_item !== 0">
          <a-badge status="processing" size="'small'" />
          待办事项：{{ getCalendarContent(current)?.todo_item }}
        </div>
        <div class="calendar-content-text" style="color: #f50;" v-if="getCalendarContent(current)?.emergency_item !== 0">
          <a-badge status="error" />
          紧急事项：{{ getCalendarContent(current)?.emergency_item }}
        </div>
      </div> -->
    </template>
  </a-calendar>
</template>
<script lang="ts" setup>
import { onMounted, ref, watch } from 'vue';
import dayjs, { Dayjs } from 'dayjs';
import { h } from 'vue';
import { LeftOutlined, RightOutlined, DownOutlined } from '@ant-design/icons-vue';
import locale from 'ant-design-vue/es/date-picker/locale/zh_CN';
import { useStore } from 'vuex';
import { acquireCalendar } from '../request';

/* 基础数据  */
// 当前日历所显示的日期
const current_date = ref<Dayjs>(dayjs('2000-1-1'));
// 日历内容接口
interface Calendar {
  calendar: string,         /* 日期 */
  solar_calendar: string,   /* 阳历day */
  lunar_calendar: string,   /* 农历day */
  rest_day_valid: number | null,  /* 是否休息日（1-是, 0-否） */
  within_month: boolean,    /* 是否是当月日期（1-是, 0-否） */
  todo_item: number,        /* 待做事项数 */
}
// 日历显示内容 
const calendar_content = ref<Map<string, Calendar>>();
// store变量
const store = useStore();


/* 构子函数 */
// 初始化函数 
onMounted(() => {
  current_date.value = dayjs(new Date());
});

// 监听当前月份的变化
watch(current_date, (newVal, oldVal) => {
  if (newVal != oldVal) {
    const timestamp = newVal.valueOf();
    // 更新全局数据
    store.commit('updateTimeStamp', timestamp);
    // 获取日历信息
    if (newVal.month() != oldVal.month()) {
      changeCalendarContent(timestamp);
    }
  }
});


/* 日历按钮功能 */
// 月份选择器发生改变
const dataPickerChange = (date: Dayjs, _dateString: string) => {
  current_date.value = date;
};

// 切换至上个月
const lastMonthChange = () => {
  current_date.value = current_date.value.add(-1, 'month');
};
// 切换至下个月
const nextMonthChange =() => {
  current_date.value = current_date.value.add(1, 'month');
}

// 返回对应日期的内容
const getCalendarContent = (value: Dayjs): Calendar | undefined => {
  let key = value.format('YYYY-MM-DD');
  return calendar_content.value?.get(key);
}

/* 对外接口 */
const changeCalendarContent = (timestamp: number) => {
  acquireCalendar(timestamp).then((result: Map<string, Calendar>) => {
    calendar_content.value = result;
  }).catch((err: any) => {
    console.log(err);
  });
}

</script>

<style scoped>
/* 月份切换按钮样式 */
.out-lined-style {
  margin-top: 5px;
  margin-right: 15px;
}
.out-lined-style button {
  color: #ffffff;
}

/* 月份选择器样式 */
::v-deep .ant-picker-input {
  width: 80%;
}
::v-deep .ant-picker-input input {
  font-weight: bold;
  font-size: 20px;
  width: 140px;
}
/* 星期栏样式 */
::v-deep .ant-picker-content th {
  text-align: center;
  font-size: 15px;
  font-weight: bold;
}
/* 取消阳历日期显示 */
::v-deep .ant-picker-calendar-date-value {
  display: none;
}
/* 内容高度 */
::v-deep .ant-picker-calendar-date-content {
  max-height: 72px;
}

/* 日历显示 */
.calendar-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.calendar-content div {
  display: inline;
}
.calendar-content-solar {
  font-size: 15px;
  /* color: #000000; */
  font-weight: bold;
  padding-left: 3%;
}
.calendar-content-rest {
  font-size: 8px;
  padding-left: 5%;
}
.calendar-content-rest-work {
  color:coral;
}
.calendar-content-rest-happy {
  color:greenyellow;
}
.calendar-content-lunar {
  font-size: 10px;
  /* font-weight: bold; */
  /* color: #000000; */
  margin-left: auto;
  margin-right: 0;
}

.calendar-content-text {
  font-size: 10px;
  font-weight: bold;
  opacity: 0.5
}

</style>