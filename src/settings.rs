use std::{
    num::NonZero,
    sync::{LazyLock, Mutex},
    thread,
};

#[derive(Debug)]
pub struct Settings {
    jobs: Mutex<NonZero<usize>>,
}

impl Settings {
    pub fn get() -> &'static Self {
        static SETTINGS: LazyLock<Settings> = LazyLock::new(Default::default);
        &SETTINGS
    }

    pub fn set_jobs(&self, jobs: NonZero<usize>) {
        *self.jobs.lock().unwrap() = jobs;
    }

    pub fn jobs(&self) -> NonZero<usize> {
        *self.jobs.lock().unwrap()
    }
}

impl Default for Settings {
    fn default() -> Self {
        let jobs = NonZero::new(thread::available_parallelism().unwrap().get()).unwrap();
        Self {
            jobs: Mutex::new(jobs),
        }
    }
}
