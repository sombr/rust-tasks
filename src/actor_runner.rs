use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::RwLock;
use std::sync::Arc;
use std::marker::Sync;
use threadpool::ThreadPool;

use tasks::Tasks;
use runnable::*;

pub struct ActorRunner<T: Runnable> {
    tasks: Arc<Tasks>,
    shutdown: AtomicBool,
    complete_status: AtomicBool,
    complete_status_latch: RwLock<bool>,

    actor: Arc<RunnableHolder<T>>,
    execution_pool: Arc<ThreadPool>
}

impl<T: Runnable> ActorRunner<T> {
    pub fn new( actor: Arc<RunnableHolder<T>>, execution_pool: Arc<ThreadPool> ) -> ActorRunner<T> {
        ActorRunner {
            tasks: Arc::new(Tasks::new()),
            shutdown: AtomicBool::new(false),
            complete_status: AtomicBool::new(false),
            complete_status_latch: RwLock::new(false),

            actor: actor,
            execution_pool: execution_pool
        }
    }

    pub fn schedule(&self) {
        self.tasks.add_task();

        let tasks_ref = self.tasks.clone();
        let actor_ref = self.actor.clone();
        self.execution_pool.execute(move || {
            while tasks_ref.fetch_task() {
                actor_ref.run();
            }
        });
    }
}
