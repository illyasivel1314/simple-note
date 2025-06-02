use std::sync::{Condvar, Mutex};

/**
 * @description: 初始任务管理器
 * @author: illya
 * @date: 2025/5/23 21:35
 **/
pub type InitialTaskMangerType = (Mutex<InitialTaskManger>, Condvar);
pub struct InitialTaskManger(pub(crate) bool);
impl Default for InitialTaskManger {
    fn default() -> Self {
        Self {
            0: Default::default(),
        }
    }
}
