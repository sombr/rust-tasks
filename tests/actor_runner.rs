extern crate threadpool;
extern crate tasks_framework;

use tasks_framework::runnable::Runnable;
use tasks_framework::actor_runner::ActorRunner;

use threadpool::ThreadPool;
use std::cell::Cell;
use std::sync::Arc;

struct Actor {
    value: Arc<Cell<usize>>
}

unsafe impl Send for Actor {}
unsafe impl Sync for Actor {}

impl Actor {
    fn new() -> Actor {
        Actor { value: Arc::new(Cell::new(0)) }
    }

    fn get_value_ref(&self) -> Arc<Cell<usize>> {
        return self.value.clone();
    }
}

impl Runnable for Actor {
    fn run(&self) {
        for _ in 0 .. 10000 {
            self.value.set( self.value.get() + 1 );
        }
    }
}

#[test]
fn actor_runner_schedule_with_a_primitive_actor_and_a_threadpool_executes_correctly() {
    let pool = Arc::new(ThreadPool::new(2));
    let actor = Actor::new();

    let val = actor.get_value_ref();

    let runner: ActorRunner<Actor> = ActorRunner::new(actor, pool);

    runner.schedule();
    runner.schedule();
    runner.schedule();

    runner.complete();

    assert_eq!( val.get(), 10000 );
}
