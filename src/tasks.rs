use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

#[allow(non_camel_case_types)]
enum State {
    WAITING,
    RUNNING_NO_TASKS,
    RUNNING_GOT_TASKS,
}

pub struct Tasks {
    state: AtomicUsize
}

impl Tasks {
    pub fn new() -> Tasks {
        Tasks { state: AtomicUsize::new(State::WAITING as usize) }
    }

    /**
     * recheck queue?
     */
    pub fn fetch_task(&self) -> bool {
        let old_state: usize = self.state.fetch_sub(1, Ordering::Relaxed);
        if old_state == State::RUNNING_GOT_TASKS as usize {
            return true;
        } else if old_state >= State::WAITING as usize && old_state <= State::RUNNING_NO_TASKS as usize {
            return false;
        } else {
            panic!(format!("unknown old state: {}", old_state));
        }
    }

    /**
     * schedule task execution?
     */
    pub fn add_task(&self) -> bool {
        // fast track for high-load applications
        // atomic get is cheaper than atomic swap
        // for both this thread and fetching thread
        if self.state.load(Ordering::Relaxed) == State::RUNNING_GOT_TASKS as usize {
            return false;
        }

        let old_state: usize = self.state.swap(State::RUNNING_GOT_TASKS as usize, Ordering::Relaxed);
        return old_state == State::WAITING as usize;
    }
}
