<template>
    <a-layout :style="{ height: '560px' }">
        <a-layout-header :style="{ background: '#141414', padding: '0px 20px 0 20px' }">
            <a-flex :justify="'space-between'" :align="'flex-start'">
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
                    <a-button type="text">
                        <template #icon>
                            <SettingOutlined :style="{color: 'white', fontSize: '20px'}"/>
                        </template>
                    </a-button>
                </a-space>
            </a-flex>
        </a-layout-header>
        <a-layout-content class="content-style scrollbar">
            <a-list 
                :bordered="false" 
                :data-source="list" 
                :split="false"
                >
                <template #renderItem="{ item, index }">
                    <a-list-item :style="{ padding: '0px' }">
                        <a-space :size="5" align="start">
                            <div class="solid-circle" />
                            <a-textarea 
                                :id="item.key"
                                v-model:value="list[index].content"
                                :autoSize="{ minRows: 1 }"
                                :bordered="false"
                                :style="{ padding: '0px 0px 10px 0px', width: '200px', height: 'auto' }"
                                @blur="updateContent(index)"
                                @pressEnter="removeFocus(item.key)"
                            />
                        </a-space>
                    </a-list-item>
                </template>
            </a-list>
        </a-layout-content>
    </a-layout>
</template>
<script setup lang="ts">
import { PlusOutlined, SettingOutlined } from '@ant-design/icons-vue';
import { computed, nextTick, ref, watch } from 'vue';
import { v4 as uuid } from 'uuid';
import store from '../store';
import { acquireNote, deleteNote, insertNote } from '../request';

/* 基础数据 */
// 便笺内容接口
interface NoteInterface {
    'key': string,              /* 唯一标识 */
    'content': string,          /* 笔记内容 */
    'timestamp': number,   /* 当前时间戳 */
}
// 计算属性 －－ 获取日历时间
const timestamp = computed(() => store.state.timeStamp);
// 笔记内容列表
const list = ref<NoteInterface[]>([]);

/* 构子函数 */
// 监听日历时间是否变更
watch(timestamp, (newValue, oldValue) => {
    if (newValue !== oldValue) {
        acquireNote(timestamp.value).then((result: NoteInterface[]) => {
            list.value = result;
        }).catch((err: any) => {
            console.log(err);
        });
    }
});

/* 其他方法 */
// 创建便笺内容
const createNewNode = () => {
    return {
        'key': uuid(),
        'content': '',
        'timestamp': timestamp.value,
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


</script>
<style>
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
    padding: 0px 20px 0 20px;
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
.solid-circle {
  width: 7px; /* 圆点的直径 */
  height: 7px;
  background-color: #ffffff; /* 圆点的颜色 */
  border-radius: 50%; /* 使div成为圆形 */
  display: inline-block; /* 使其可以与其他元素并排显示 */
}
</style>