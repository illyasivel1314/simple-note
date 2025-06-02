<template>
    <a-layout :style="{ height: '560px' }">
        <a-layout-header :style="{ background: '#141414', padding: '0px 20px 0 20px', height: '55px' }">
            <a-flex :justify="'space-between'" :align="'flex-start'" :style="{ height: '100%' }">
                <!-- 显示标题 -->
                <div class="header-text-style">
                    Todo
                </div>
                <a-space :size="10">
                    <!-- 添加文本按钮 -->
                    <a-button type="text" @click="addNoteText">
                        <template #icon>
                            <PlusOutlined :style="{color: 'white', fontSize: '20px'}"/>
                        </template>
                    </a-button>
                    <!-- 设置按钮 -->
                    <a-button type="text" @click="">
                        <template #icon>
                            <SettingOutlined :style="{color: 'white', fontSize: '20px'}"/>
                        </template>
                    </a-button>
                </a-space>
            </a-flex>
        </a-layout-header>
        <a-layout-content class="content-style scrollbar" :style="{ height: '100%' }">
            <a-list :bordered="false" :data-source="list" :split="false">
                <template #renderItem="{ item, index }">
                    <a-list-item :style="{ padding: '0px' }">
                        <a-input-group>
                            <a-flex :justify="'justify'" :align="'flex-start'" :style="{ padding: '5px 0px 2px 0px' }">
                                <!-- 标记点 -->
                                <a-badge class="solid-circle-style" color="#ffffff" />
                                <!-- 标签 -->
                                <a-tag class="tag-style" :bordered="false" :color="compontTag[index].color">
                                    {{ compontTag[index].content }}
                                </a-tag>
                                <!-- 右键菜单 -->
                                <a-dropdown :trigger="['contextmenu']">
                                    <!-- 输入框 -->
                                    <a-textarea 
                                        :id="item.key"
                                        :class="item.tag_content !== null ? 'inner-textarea-style' : ''"
                                        v-model:value="list[index].content"
                                        :autoSize="{ minRows: 1 }"
                                        :bordered="false"
                                        :style="{ padding: '0px 0px 0px 0px', width: '100%', height: 'auto' }"
                                        @blur="updateContent(index)"
                                        @pressEnter="removeFocus(item.key)"
                                        @contextmenu.prevent=removeFocus(item.key)
                                    />
                                    <!-- 菜单栏 -->
                                    <template #overlay>
                                        <a-menu>
                                            <a-menu-item key="1" v-if="list[index].finished_valid == 0" @click="archiveNote(index)">完成</a-menu-item>
                                            <a-menu-item key="2" v-else @click="archiveNote(index)">未完成</a-menu-item>
                                            <a-menu-item key="4" @click="removeNote(index)">删除</a-menu-item>
                                            <a-menu-item key="3" @click="setConfiguration(index)">更多设置</a-menu-item>
                                        </a-menu>
                                    </template>
                                </a-dropdown>
                            </a-flex>
                            <!-- 设置卡片 -->
                            <a-card style="width: 100%" v-show="list[index].setting">
                                <a-form :label-col="{ style: { width: '65px' }}">
                                    <!-- 执行方式 -->
                                    <a-form-item label="执行方式">
                                        <a-radio-group v-model:value="list[index].tag_type" size="small" :change="changeCategory(index)">
                                            <a-radio class="radio-group-style" :value=0>单次</a-radio>
                                            <a-radio class="radio-group-style" :value=1>周期</a-radio>
                                        </a-radio-group>
                                    </a-form-item>
                                    <!-- 执行时间 -->
                                    <a-form-item v-show="list[index].tag_type == 1" label="便笺周期">
                                        <a-range-picker
                                            v-model:value="list[index].execute_stamp" 
                                            class="time-picker-style"
                                            :format="'MM/DD'" 
                                            :size="'small'"
                                            :bordered="false"
                                            :allowClear="false"
                                        />
                                    </a-form-item>
                                    <!-- 是否提醒 -->
                                    <a-form-item label="是否提醒">
                                        <a-radio-group v-model:value="list[index].reminder_valid" size="small" :change="changeReminder(index)">
                                            <a-radio class="radio-group-style" :value=0>否</a-radio>
                                            <a-radio class="radio-group-style" :value=1>是</a-radio>
                                        </a-radio-group>
                                    </a-form-item>
                                    <!-- 提醒时间 -->
                                    <a-form-item v-show="list[index].reminder_valid == 1" label="提醒时间">
                                        <a-date-picker v-model:value="list[index].reminder_stamp"
                                            class="time-picker-style"
                                            format="MM-DD HH:mm"
                                            :show-time="{ 
                                                hideDisabledOptions: true, 
                                                format: 'HH:mm', 
                                                minuteStep: 5 
                                            }"
                                            :bordered="false"
                                            :allowClear="false"
                                            :size="'small'"  
                                            :disabled-date="disabledDate"
                                            :disabled-time="disabledTime"
                                            :showNow="false"
                                        />
                                    </a-form-item>
                                    <!-- 按钮组 -->
                                    <a-form-item>
                                        <a-flex :style="{width: '100%', height: '20px', padding: '5px'}" :justify="'space-around'" :align="'center'">
                                            <a-button class="card-button-style" @click="enterSettingCancel(index)">
                                                <template #icon><CloseOutlined style="font-size: 10px;"/></template>
                                                <span style="font-size: 10px;">取消</span>
                                            </a-button>
                                            <a-button class="card-button-style" @click.prevent="enterSettingSubmit(index)">
                                                <template #icon><CheckOutlined style="font-size: 10px;"/></template>
                                                <span style="font-size: 10px;">提交</span>
                                            </a-button>
                                        </a-flex>
                                    </a-form-item>
                                </a-form>
                            </a-card>
                        </a-input-group>
                    </a-list-item>
                </template>
            </a-list>
        </a-layout-content>
    </a-layout>
</template>
<script setup lang="ts">
import { PlusOutlined, SettingOutlined, CheckOutlined, CloseOutlined } from '@ant-design/icons-vue';
import { computed, nextTick, ref, watch } from 'vue';
import { v4 as uuid } from 'uuid';
import store from '../store';
import { acquireNote, deleteNote, insertNote, updateNoteSetting, updateNoteFinish, acquireNoteByKey } from '../request';
import dayjs, { Dayjs } from 'dayjs';
import { NoteInterface } from '../request/interface';
import { message } from 'ant-design-vue';

/* 基础数据 */
// 计算属性 －－ 获取日历时间
const timestamp = computed(() => store.state.timeStamp);
// 笔记内容列表
const list = ref<NoteInterface[]>([]);
const oldNote = ref<NoteInterface | null>(null);

/* 构子函数 */
// 监听日历时间是否变更
watch(timestamp, async (newValue, oldValue) => {
    if (newValue !== oldValue) {
        list.value = await acquireNote(timestamp.value);
    }
});

/* 其他方法 */
// 创建便笺内容
const createNewNode = () => {
    return {
        key: uuid(),
        content: '',

        setting: false,
        finished_valid: 0 as 0 | 1,

        tag_type: 0 as 0 | 1,
        tag_content: true,
        execute_stamp: [dayjs(timestamp.value), dayjs(timestamp.value)] as [Dayjs, Dayjs],

        reminder_valid: 0 as 0 | 1,
        reminder_stamp: null,
    };
}

/* 便笺方法 */
// 增加新便笺
const addNoteText = () => {
    // 初始化便笺数据
    let new_note = createNewNode();
    list.value.push(new_note);
    // 获取新便笺焦点
    nextTick(() => {
        document.getElementById(new_note.key)?.focus();
    });
}
// 更新数据
const updateContent = ((index: number) => {
    list.value[index].tag_content = false;
    let new_note = list.value[index];
    if (new_note == null) {
        return;
    }
    // 清除用户输入的多余空格
    list.value[index].content = list.value[index].content.trimEnd();
    // 用户输入内容为空
    if (list.value[index].content.length === 0) {
        deleteNote(list.value[index].key);
        list.value.splice(index, 1);
        return;
    }
    // 调用外部接口，保存数据
    insertNote(list.value[index]);
});

// 按下回车键后，移除焦点
const removeFocus = ((key: string) => {
    nextTick(() => {
        document.getElementById(key)?.blur();
    });
});

// 打开设置
const setConfiguration = ((index: number) => {
  list.value[index].setting = true;
  // 保存旧值
  oldNote.value = list.value[index];
});


const compontTag = computed(() => {
    let tagList: { color: string, content: string}[] = [];
    list.value.forEach((note: NoteInterface) => {
        let now = dayjs();
        if (note.tag_content == true) {
            tagList.push({
                color: 'cyan',
                content: '新增'
            });
        } else if (note.finished_valid === 1) {
            tagList.push({
                color: 'success',
                content: '已完成'
            });
        } else if (dayjs(note.execute_stamp[0]).startOf('day') > now.endOf('day')) { 
            tagList.push({
                color: 'default',
                content: '未开始'
            });
        } else if (dayjs(note.execute_stamp[1]).endOf('day') < now.startOf('day')) {
            tagList.push({
                color: 'error',
                content: '已过期'
            });
        } else if (note.tag_type === 0) {
            tagList.push({
                color: 'processing',
                content: '进行中'
            });
        } else if (note.tag_type === 1) {
            let dayDiff = note.execute_stamp[1].diff(note.execute_stamp[0], 'day') + 1;
            tagList.push({
                color: 'warning',
                content: dayDiff + '天'
            });
        }
    });
    return tagList;
});

/* 不能选择之前的时间 */
const disabledDate = (current: Dayjs) => {
    return current && current < dayjs().startOf('day');
};
const disabledTime = (current: Dayjs) => {
  const now = dayjs(); // 当前时间
  // 如果选择的日期是今天，则禁用当前时间之前的时间
  if (current && current.isSame(now, 'day')) {
    return {
      disabledHours: () => range(0, now.hour()),
    };
  }
  
  // 如果不是今天，不禁用任何时间
  return {
    disabledHours: () => [],
    disabledMinutes: () => [],
  };
};
// 辅助函数：生成数字范围
const range = (start: number, end: number) => {
  const result = [];
  for (let i = start; i < end; i++) {
    result.push(i);
  }
  return result;
};

// 修改执行方式
const changeCategory = (index: number) => {
  // 单次的执行周期都是今天
    if (list.value[index].tag_type === 0) {
        list.value[index].execute_stamp = [dayjs(timestamp.value), dayjs(timestamp.value)] as [Dayjs, Dayjs]
    }
}
// 修改是否提醒时间
const changeReminder = (index: number) => {
    if (list.value[index].reminder_valid === 0) {
        list.value[index].reminder_stamp = null;
    }
}

// 归档
const archiveNote = (index: number) => {
    list.value[index].finished_valid = list.value[index].finished_valid === 0 ? 1 : 0;
    // 调用后端接口
    updateNoteFinish(list.value[index].key, list.value[index].finished_valid);
}

// 移除
const removeNote = (index: number) => {
    deleteNote(list.value[index].key);
    list.value.splice(index, 1);
}

// 取消配置
const enterSettingCancel= async (index: number) => {
    let value = await acquireNoteByKey(list.value[index].key);
    if (value != null) {
      list.value[index] = value;
    }
    list.value[index].setting = false;
}

// 提交配置
const enterSettingSubmit = (index: number) => {

    let note = list.value[index];
    if (note.reminder_valid === 1 && note.reminder_stamp == null) {
        message.error('请设置提醒时间');
        return;
    } else if (note.reminder_valid === 1 && dayjs().add(5, "minute").isAfter(note.reminder_stamp)) {
        message.error('提醒时间过近...');
        return;
    }
    
    let current = dayjs(timestamp.value);
    if (note.execute_stamp[0].startOf('day') > current || note.execute_stamp[1].endOf('day') < current) {
        message.error('执行周期应当包含当前日期');
        return;
    }
    // 调用后端接口
    updateNoteSetting(note);
    list.value[index].setting = false;
}



</script>
<style scoped>
/* 内容显示 */
.header-text-style {
    font-size: 20px;
    font-weight: bold;
}
/* 图标显示 */
.header-icon-style {
    width: 30%;
}

/* 内容显示样式 */
.content-style {
    padding: 0px 10px 0 20px;
}

/* 滚动条 */
.scrollbar {
  overflow: auto;
  height: 100%;
  width: 100%;
}
 
.scrollbar::-webkit-scrollbar {
  width: 8px;
}

/* textarea前的圆点 */
.solid-circle-style {
  width: 15px;
  margin-right: auto;
  display: inline-block; /* 使其可以与其他元素并排显示 */
}

/* 设置标签文字大小 */
.tag-style {
  margin: 0px 5px 0px 0px;
  width: 53px; 
  text-align: center;
}

/* textarea段前空格 */
.inner-textarea-style {
  text-indent: 60px;
  margin: 0px 0px 0px -60px; 
}

/* 取消card的padding */
::v-deep .ant-card-body {
  padding: 0px 5px 10px 5px;
}

/* 设置字体大小 */
.time-picker-style {
  font-size: 13px;
  width: 140px;
}

/* 设置日期选择器的文本大小 */
::v-deep .ant-card-body input {
  font-size: 12px;
}

/* 表单大小 */
::v-deep .ant-form-item {
    margin: 0px;
    height: 20px;
}

/* 表单文本大小 */
::v-deep .ant-form-item-label label {
    font-size: 12px;
}

/* 单选框文本 */
::v-deep .ant-radio-inner {
  width: 13px;
  height: 13px;
}

/* 单选框文本 */
.radio-group-style {
  font-size: 11px;
}

/* 按钮文本 */
.card-button-style {
  width: 45%;
  height: 15px;
  padding: 2px 0px 0px 0px;
  display: flex;
  justify-content: center;
  align-items: center;
}

</style>