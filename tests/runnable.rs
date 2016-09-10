extern crate threadpool;
extern crate tasks;

use tasks::runnable::Runnable;
use tasks::actor_runner::ActorRunner;

use threadpool::ThreadPool;
use std::cell::Cell;
use std::sync::Arc;

struct Actor {
    value: Cell<usize>
}

unsafe impl Send for Actor {}
unsafe impl Sync for Actor {}

impl Actor {
    fn new() -> Actor {
        Actor { value: Cell::new(0) }
    }
}

impl Runnable for Actor {
    fn run(&self) {
        for _ in 0..100 {
            self.value.set( self.value.get() + 1 );
            println!(">> {}", self.value.get());
        }
    }
}

#[test]
fn test_fetch_task_should_not_be_called_on_a_waiting_state() {
    let pool = Arc::new(ThreadPool::new(2));
    let actor = Actor::new();

    let runner: ActorRunner<Actor> = ActorRunner::new(actor, pool);

    runner.schedule();
    runner.schedule();
}
