pub struct ThreadPool {}

impl ThreadPool {
    pub fn new(count: i32) -> Self {
        println!("creating a ThreadPool with count {count}");
        ThreadPool {}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: Fn() -> (),
    {
        println!("executing thread");
        f();
    }
}
