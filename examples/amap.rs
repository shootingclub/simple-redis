use std::collections::HashMap;
use std::fmt::Display;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct AmapMetrics {
    pub data: Arc<HashMap<String, AtomicI64>>,
}

impl AmapMetrics {
    pub fn _new(metrics_name: Vec<String>) -> Self {
        let data = metrics_name
            .iter()
            .map(|name| (name.to_string(), AtomicI64::new(0)))
            .collect::<HashMap<String, AtomicI64>>();
        AmapMetrics {
            data: Arc::new(data),
        }
    }
    pub fn _get(&self, key: &str) -> Option<i64> {
        self.data.get(key).map(|v| v.load(Ordering::Relaxed))
    }
    pub fn _inc(&self, key: &str) {
        if let Some(v) = self.data.get(key) {
            v.fetch_add(1, Ordering::Relaxed);
        }
    }
    pub fn _dec(&self, key: &str) {
        if let Some(v) = self.data.get(key) {
            v.fetch_sub(1, Ordering::Relaxed);
        }
    }

    pub fn _snapshot(&self) -> HashMap<String, i64> {
        self.data
            .iter()
            .map(|(k, v)| (k.to_string(), v.load(Ordering::Relaxed)))
            .collect()
    }
}

impl Display for AmapMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, v) in self.data.iter() {
            writeln!(f, "{} = {}", k, v.load(Ordering::Relaxed))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_amap_metrics() {
        let metrics = AmapMetrics::_new(vec!["test".to_string()]);
        println!("metrics: {:?}", metrics._snapshot());
        for i in 0..4 {
            let clone = metrics.clone();
            thread::Builder::new()
                .name(format!("{}", format_args!("worker-{}", i)))
                .spawn(move || {
                    for _ in 0..100 {
                        clone._inc("test");
                        println!(
                            "{:?},value: {:?}",
                            thread::current().name().unwrap(),
                            clone._get("test").unwrap()
                        )
                    }
                })
                .expect("thread spawn fail");
        }
        thread::sleep(Duration::from_secs(1));
        println!("metrics: {:?}", metrics._snapshot());
    }
}

#[allow(dead_code)]
fn main() {}
