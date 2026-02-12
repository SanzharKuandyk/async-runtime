use std::task::{Context, Poll};

use crate::scheduler::TaskQueue;

pub struct Executor {
    queue: TaskQueue,
}

impl Executor {
    pub fn new(queue: TaskQueue) -> Self {
        Self { queue }
    }

    pub fn run(&self) {
        while let Some(task) = self.queue.pop() {
            let waker = task.waker(self.queue.clone());
            let mut cx = Context::from_waker(&waker);

            // lock the future so we get &mut access
            let mut future = task.future.lock().unwrap();

            match future.as_mut().poll(&mut cx) {
                Poll::Pending => { /* do nothing, waker handles rescheduling */ }
                Poll::Ready(()) => { /* finished */ }
            }
        }
    }
}
