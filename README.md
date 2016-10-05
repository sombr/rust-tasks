# Simple Rust Actors
Actors framework (inspired by a similar work of Stepan Kolcov in Java) with a Lock-Free message queue and a threadpool.

Actor starts processing as soon  as any workload added and shutdowns when you call "complete".

Substantial performance benefit comes from  using a lock-free collection for input tasks ("messages") and  only scheduling a new actor execution in a separate thread when there's a gap in workload input. 

## Usage

To use the framework, you need to implement ``MessageProcessor`` trait for your data type.

```rust
extern crate threadpool;
extern crate tasks_framework;

use threadpool::ThreadPool;

use tasks_framework::single_queue_actor::MessageProcessor;
use tasks_framework::single_queue_actor::SingleQueueActor;

struct NewMessageType {
   ... your custome message type ...
}

struct SimpleMessageProcessor {
   ...
}

impl MessageProcessor<NewMessageType> for SimpleMessageProcessor {
    fn process_message(&self, message: NewMessageType) {
       ... message processing logic - executed asyncronously in the threadpool ...
    }
}

let pool = Arc::new(ThreadPool::new(2));
    let processor = Arc::new(SimpleMessageProcessor::new());

    let simple_actor = SingleQueueActor::new( processor.clone(), pool );

    ... add some work ...
        simple_actor.add_message(message_to_process);
    ... which starts to be processed immediately when the first message arrives ...
    
    ... to "shutdown" the actor and wait for the completion of any in-progress tasks.
    simple_actor.complete();

```
