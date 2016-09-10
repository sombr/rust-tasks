extern crate threadpool;
extern crate tasks_framework;

use tasks_framework::single_queue_actor::MessageProcessor;
use tasks_framework::single_queue_actor::SingleQueueActor;

use threadpool::ThreadPool;
use std::cell::Cell;
use std::sync::Arc;

struct SimpleMessageProcessor {
    processed: Cell<usize>
}

unsafe impl Send for SimpleMessageProcessor {}
unsafe impl Sync for SimpleMessageProcessor {}

impl SimpleMessageProcessor {
    fn new() -> SimpleMessageProcessor {
        SimpleMessageProcessor { processed: Cell::new(0) }
    }

    fn get_processed(&self) -> usize {
        return self.processed.get();
    }
}

impl MessageProcessor<usize> for SimpleMessageProcessor {
    fn process_message(&self, _: usize) {
        self.processed.set( self.processed.get() + 1 );
    }
}

#[test]
fn actor_runner_schedule_with_a_primitive_actor_and_a_threadpool_executes_correctly() {
    let pool = Arc::new(ThreadPool::new(2));
    let processor = Arc::new(SimpleMessageProcessor::new());

    let simple_actor = SingleQueueActor::new( processor.clone(), pool );

    for i in 0 .. 10000 {
        simple_actor.add_message(i);
    }
    simple_actor.complete();

    assert_eq!(simple_actor.get_queue_size(), 0);
    assert_eq!(processor.get_processed(), 10000);
}
