use std::sync::Arc;

use crossbeam_queue::SegQueue;

use crate::task::Task;

#[derive(Clone)]
pub struct TaskQueue(pub Arc<SegQueue<Arc<Task>>>);

impl TaskQueue {
    pub fn new() -> Self {
        TaskQueue(Arc::new(SegQueue::new()))
    }

    pub fn push(&self, task: Arc<Task>) {
        self.0.push(task)
    }

    pub fn pop(&self) -> Option<Arc<Task>> {
        self.0.pop()
    }
}
