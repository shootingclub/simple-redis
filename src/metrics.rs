use dashmap::DashMap;
use rand::Rng;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Metrics {
    pub data: Arc<DashMap<String, i64>>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data: Arc::new(DashMap::new()),
        }
    }
    pub fn get(&self, key: &str) -> Option<i64> {
        self.data.get(key).map(|v| *v.value())
    }
    pub fn inc(&self, key: &str) {
        self.data
            .entry(key.to_string())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    pub fn dec(&mut self, key: &str) {
        self.data
            .entry(key.to_string())
            .and_modify(|e| *e -= 1)
            .or_insert(0);
    }
    pub fn snapshot(&self) -> HashMap<String, i64> {
        self.data
            .iter()
            .map(|e| (e.key().clone(), *e.value()))
            .collect()
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
    use dashmap::DashMap;

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

    #[test]
    fn test_metrics_dash_map() {
        let map = DashMap::<String, i64>::new();
        map.insert("sss".to_string(), 2);
    }

    #[test]
    fn test_metrics_vec_hash() {
        let array = [1, 23, 4, 5, 6, 7, 8, 9, 111, 12, 4];
        array.iter().for_each(|v| {
            println!("{}", v);
        });
        let map = array
            .iter()
            .map(|v| (v.to_string(), v))
            .collect::<HashMap<String, &i32>>();
        println!("{:?}", map);

        #[derive(Debug)]
        struct Hmm {
            // key
            _key: String,
            // value
            _val: i64,
        }

        let mut hmap = HashMap::<String, i64>::new();
        hmap.insert("sss".to_string(), 2);
        hmap.insert("kk".to_string(), 2);
        hmap.insert("ll".to_string(), 2);
        let hmm_array = hmap
            .iter()
            .map(|v| Hmm {
                _key: v.0.to_string(),
                _val: *v.1,
            })
            .collect::<Vec<Hmm>>();
        println!("{:?}", hmm_array);
        let hmm_array = hmap
            .iter()
            .map(|v| (v.0.to_string(), *v.1))
            .collect::<Vec<(String, i64)>>();
        println!("{:?}", hmm_array)
    }
}
