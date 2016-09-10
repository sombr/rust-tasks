use lock_free_stack::lock_free_stack::Stack;
use actor_runner::ActorRunner;

use std::sync::Arc;
use std::marker::Sync;
use threadpool::ThreadPool;
use runnable::Runnable;

pub trait MessageProcessor<T> {
    fn process_message(&self, message: T);
}

struct ActorImpl<T: 'static, M: MessageProcessor<T> + Send + Sync + 'static> {
    message_processor: Arc<M>,
    queue: Arc<Stack<T>>,
}

impl<T: 'static, M: MessageProcessor<T> + Send + Sync + 'static> Runnable for ActorImpl<T, M> {
    fn run(&self) {
        for message in self.queue.remove_all() {
            self.message_processor.process_message(message);
        }
    }
}

pub struct SingleQueueActor<T: 'static, M: MessageProcessor<T> + Send + Sync + 'static> {
    queue: Arc<Stack<T>>,
    actor_runner: ActorRunner<ActorImpl<T, M>>
}

impl<T: 'static, M: MessageProcessor<T> + Send + Sync + 'static> SingleQueueActor<T, M> {
    pub fn new( message_processor: Arc<M>, execution_pool: Arc<ThreadPool> ) -> SingleQueueActor<T, M> {
        let queue_arc = Arc::new(Stack::new());
        let actor_impl = ActorImpl {
            message_processor: message_processor,
            queue: queue_arc.clone()
        };

        SingleQueueActor {
            queue: queue_arc,
            actor_runner: ActorRunner::new(actor_impl, execution_pool)
        }
    }

    pub fn get_queue_size(&self) -> usize {
        self.queue.size()
    }

    pub fn add_message(&self, message: T) {
        self.queue.add(message);
        self.actor_runner.schedule();
    }

    pub fn complete(&self) {
        self.actor_runner.complete();
    }
}
