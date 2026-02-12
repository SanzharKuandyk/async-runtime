use std::sync::Mutex;
use std::{future::Future, pin::Pin, sync::Arc, task::Waker};

use crate::{scheduler::TaskQueue, waker::waker_from_arc};

pub struct Task {
    pub future: Mutex<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + Send + 'static) -> Arc<Task> {
        Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
        })
    }

    // Creates a waker that can push this task back to the queue
    pub fn waker(self: &Arc<Self>, queue: TaskQueue) -> Waker {
        waker_from_arc(self.clone(), queue)
    }
}
