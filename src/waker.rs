use std::{
    sync::Arc,
    task::{RawWaker, RawWakerVTable, Waker},
};

use crate::{scheduler::TaskQueue, task::Task};

struct WakerData {
    task: Arc<Task>,
    queue: TaskQueue,
}

pub fn waker_from_arc(task: Arc<Task>, queue: TaskQueue) -> Waker {
    let data = Arc::new(WakerData { task, queue });

    unsafe { Waker::from_raw(RawWaker::new(Arc::into_raw(data) as *const (), &VTABLE)) }
}

unsafe fn clone(data: *const ()) -> RawWaker {
    let arc: Arc<WakerData> = Arc::from_raw(data as *const WakerData);
    let cloned = arc.clone();
    std::mem::forget(arc);
    RawWaker::new(Arc::into_raw(cloned) as *const (), &VTABLE)
}

unsafe fn wake(data: *const ()) {
    let arc: Arc<WakerData> = Arc::from_raw(data as *const WakerData);
    arc.queue.push(arc.task.clone());
}

unsafe fn wake_by_ref(data: *const ()) {
    let arc: Arc<WakerData> = Arc::from_raw(data as *const WakerData);
    arc.queue.push(arc.task.clone());
    std::mem::forget(arc);
}

unsafe fn drop(data: *const ()) {
    // Recreate the Arc so it decreases the refcount.
    let _ = Arc::from_raw(data as *const WakerData);
    // No need to explicitly call drop(); Rust drops `_` at end of scope.
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
