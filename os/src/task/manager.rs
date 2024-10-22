//!Implementation of [`TaskManager`]
use super::{TaskControlBlock, BIG_STRIDE};
use crate::sync::UPSafeCell;
use alloc::sync::Arc;
use alloc::vec::Vec;
use lazy_static::*;

///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: Vec<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: Vec::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        if self.ready_queue.is_empty() {
            return None;
        }

        let mut min_stride = 1_000_000_007;
        let mut index = 0;
        {
            for i in 0..self.ready_queue.len() {
                let new_stride = self.ready_queue[i].inner_exclusive_access().stride;
                if new_stride <= min_stride {
                    min_stride = new_stride;
                    index = i;
                }
            }
        }

        let tcb = self.ready_queue[index].clone();
        self.ready_queue.remove(index);
        {
            let mut inner = tcb.inner_exclusive_access();
            let pass = BIG_STRIDE / inner.priority;
            inner.stride += pass;
        }

        Some(tcb)
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}
