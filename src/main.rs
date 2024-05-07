use simple_redis::{request_worker, task_worker, Metrics, M, N};
use std::thread;
use std::time::Duration;

fn main() {
    let metrics = Metrics::new();
    println!("{:?}", metrics.snapshot());
    for idx in 0..N {
        task_worker(idx, metrics.clone());
    }
    for _ in 0..M {
        request_worker(metrics.clone());
    }
    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{:?}", metrics.snapshot());
    }
}
