extern crate tasks;
use tasks::tasks::Tasks;

use std::thread::JoinHandle;
use std::thread::spawn;
use std::sync::Arc;

#[test]
#[should_panic]
fn test_fetch_task_should_not_be_called_on_a_waiting_state() {
    let tasks = Tasks::new();
    let recheck_queue = tasks.fetch_task();
    assert_eq!(recheck_queue, false)
}

#[test]
fn test_add_task_returns_schedule_for_execution_true_for_a_waiting_state() {
    let tasks = Tasks::new();
    let schedule_for_execution = tasks.add_task();
    assert_eq!(schedule_for_execution, true)
}

#[test]
fn test_add_task_returns_schedule_for_execution_false_for_an_in_flight_state_with_tasks() {
    let tasks = Tasks::new();
    let _ = tasks.add_task();

    let schedule_for_execution = tasks.add_task();
    assert_eq!(schedule_for_execution, false)
}

#[test]
fn test_fetch_task_returns_recheck_queue_true_for_an_in_flight_state_with_tasks() {
    let tasks = Tasks::new();
    let _ = tasks.add_task();

    let recheck_queue = tasks.fetch_task();
    assert_eq!(recheck_queue, true)
}

#[test]
fn test_add_task_returns_schedule_for_execution_false_for_an_in_flight_state_without_tasks() {
    let tasks = Tasks::new();
    let _ = tasks.add_task();
    let _ = tasks.fetch_task();

    let schedule_for_execution = tasks.add_task();
    assert_eq!(schedule_for_execution, false)
}

#[test]
fn test_fetch_task_returns_recheck_queue_false_for_an_in_flight_state_without_tasks() {
    let tasks = Tasks::new();
    let _ = tasks.add_task();
    let _ = tasks.fetch_task();

    let recheck_queue = tasks.fetch_task();
    assert_eq!(recheck_queue, false)
}

#[test]
fn test_multi_thread() {
    let tasks: Arc<Tasks> = Arc::new(Tasks::new());

    let mut threads: Vec<JoinHandle<()>> = vec!();
    for i in 0 .. 10 {
        let tasks_ref = tasks.clone();
        threads.push(spawn(move || {
            for j in 1 .. 10 {
                if j % 2 == 0 {
                    let _ = tasks_ref.fetch_task();
                } else {
                    let _ = tasks_ref.add_task();
                }
            }
        }));
    }

    for handle in threads {
        let _ = handle.join();
    }

    assert!(true);
}
