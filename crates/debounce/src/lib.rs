use std::time::{Duration, Instant};
pub struct Bouncer<T> {
    pub delay: Duration,
    last_run: Option<Instant>,
    func: Option<fn() -> T>,
    result: Option<T>,
}

impl<T> Bouncer<T> {
    pub fn new(delay: Duration) -> Self {
        Self {
            delay,
            last_run: None,
            func: None,
            result: None,
        }
    }

    pub fn with_func(mut self, func: fn() -> T) -> Self {
        self.func = Some(func);
        self
    }

    pub fn execute(&mut self) {
        if let Some(func) = self.func {
            let result = self.debounce(func);
            self.result = result;
        }
    }

    pub fn get_result(&mut self) -> Option<&T> {
        self.result.as_ref()
    }

    pub fn debounce(&mut self, func: fn() -> T) -> Option<T> {
        match self.last_run {
            Some(last_run) => {
                let now = Instant::now();

                if now.duration_since(last_run) > self.delay {
                    self.last_run = Some(Instant::now());

                    Some(func())
                } else {
                    None
                }
            }
            None => {
                self.last_run = Some(Instant::now());
                Some(func())
            }
        }
    }

    pub fn reset(&mut self) {
        self.last_run = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let delay = Duration::from_secs(1);
        let mut bouncer = Bouncer::new(delay);

        let result = bouncer.debounce(|| 5 + 6);

        assert!(result.is_some());
        assert_eq!(result.unwrap(), 11);
    }

    #[test]
    fn it_binds_internal_func() {
        let func = || 5 + 6;
        let delay = Duration::from_secs(1);

        let mut bouncer = Bouncer::new(delay).with_func(func);

        assert!(bouncer.get_result().is_none());
        bouncer.execute();
        assert!(bouncer.get_result().is_some());
    }

    #[test]
    fn it_debounces() {
        let delay = Duration::from_millis(100);
        let mut bouncer = Bouncer::new(delay);

        let func = || 5 + 6;

        let result1 = bouncer.debounce(func);
        let result2 = bouncer.debounce(func);

        std::thread::sleep(Duration::from_millis(101));

        let result3 = bouncer.debounce(func);

        assert!(result1.is_some());
        assert_eq!(result1.unwrap(), 11);
        assert!(result2.is_none());
        assert!(result3.is_some());
        assert_eq!(result3.unwrap(), 11);
    }
}
