import { createStore } from 'vuex';

/**
 * 创建仓库和导出
 */
export default createStore({
    // 存储数据
    state: {
        timeStamp: 0, /* 当前时间 */
    },
    // 更新存储数据的值
    mutations: {
        updateTimeStamp(state, value) {
            state.timeStamp = value;
        }
    },
    // 异步操作在action中进行，再传递到mutation
    actions: {
    }
});