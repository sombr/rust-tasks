pub trait Runnable {
    fn run(&self);
}

pub struct RunnableHolder<T: Runnable> {
    runnable: T
}

impl<T: Runnable> RunnableHolder<T> {
    pub fn run(&self) {
        self.runnable.run();
    }
}
