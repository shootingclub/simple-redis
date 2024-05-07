use rand::Rng;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Metrics {
    pub data: Arc<Mutex<HashMap<String, i64>>>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn get(&self, key: &str) -> Option<i64> {
        let data = self.data.lock().unwrap();
        data.get(key).cloned()
    }
    pub fn inc(&self, key: &str) {
        let mut data = self.data.lock().unwrap();
        data.entry(key.to_string())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    pub fn dec(&mut self, key: &str) {
        let mut data = self.data.lock().unwrap();
        data.entry(key.to_string())
            .and_modify(|e| *e -= 1)
            .or_insert(0);
    }
    pub fn snapshot(&self) -> HashMap<String, i64> {
        self.data.lock().unwrap().clone()
    }
}

pub fn task_worker(idx: usize, metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
        metrics.inc(format!("call.thread.worker.{}", idx).as_str());
    });
}

pub fn request_worker(metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
        let page = rng.gen_range(1..5);
        metrics.inc(format!("req.page.{}", page).as_str());
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_inc_dec() {
        let mut metrics = Metrics::new();
        metrics.inc("test");
        println!("{:?}", metrics.get("test"));
        metrics.inc("test");
        println!("{:?}", metrics.get("test"));
        metrics.dec("test");
        println!("{:?}", metrics.get("test"));
    }

    #[test]
    fn test_metrics_snapshot() {
        let mut metrics = Metrics::new();
        metrics.inc("test");
        metrics.inc("test");
        metrics.dec("test");
        println!("{:?}", metrics.snapshot());
    }

    #[test]
    fn test_metrics_thread_worker() {
        let metrics = Metrics::new();
        println!("metrics: {:?}", metrics.snapshot());
        for i in 0..4 {
            let clone = metrics.clone();
            thread::Builder::new()
                .name(format!("{}", format_args!("worker-{}", i)))
                .spawn(move || {
                    for _ in 0..100 {
                        clone.inc("test");
                        println!(
                            "{:?},value: {:?}",
                            thread::current().name().unwrap(),
                            clone.get("test").unwrap()
                        )
                    }
                })
                .expect("thread spawn fail");
        }
        thread::sleep(Duration::from_secs(1));
        println!("metrics: {:?}", metrics.snapshot());
    }
}
