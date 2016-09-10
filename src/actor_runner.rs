use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Barrier};
use std::marker::Sync;
use threadpool::ThreadPool;

use tasks::Tasks;
use runnable::Runnable;

pub struct ActorRunner<T: Runnable + Send + Sync + 'static> {
    tasks: Arc<Tasks>,
    shutdown: Arc<AtomicBool>,
    complete_status: Arc<Barrier>,

    actor: Arc<T>,
    execution_pool: Arc<ThreadPool>
}

impl<T: Runnable + Send + Sync + 'static> ActorRunner<T> {
    pub fn new( actor: T, execution_pool: Arc<ThreadPool> ) -> ActorRunner<T> {
        ActorRunner {
            tasks: Arc::new(Tasks::new()),
            shutdown: Arc::new(AtomicBool::new(false)),
            complete_status: Arc::new(Barrier::new(2)),

            actor: Arc::new(actor),
            execution_pool: execution_pool
        }
    }

    pub fn schedule(&self) {
        if self.tasks.add_task() {
            let tasks_ref = self.tasks.clone();
            let actor_ref = self.actor.clone();
            let shutdown_ref = self.shutdown.clone();
            let complete_status_ref = self.complete_status.clone();

            self.execution_pool.execute(move || {
                while tasks_ref.fetch_task() {
                    actor_ref.run();

                    if shutdown_ref.load(Ordering::Relaxed) {
                        complete_status_ref.wait();
                    }
                }
            })
        }
    }

    pub fn complete(&self) {
        self.shutdown.store(true, Ordering::Relaxed);
        self.schedule();

        self.complete_status.wait();
    }
}

impl<T: Runnable + Send + Sync + 'static> Drop for ActorRunner<T> {
    fn drop(&mut self) {
        self.complete();
    }
}
