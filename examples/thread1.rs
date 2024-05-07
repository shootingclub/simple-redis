use anyhow::Result;
use std::sync::mpsc;
use std::thread;

// produce consumer 模型

#[derive(Debug)]
struct Msg {
    ids: usize,

    value: usize,
}

impl Msg {
    fn new(ids: usize, value: usize) -> Self {
        Self { ids, value }
    }
}

const THREAD_NUM: usize = 4;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    for i in 0..THREAD_NUM {
        let tx = tx.clone();
        thread::Builder::new()
            .name(format!("thread-{}", i))
            .spawn(move || producer(i, tx))
            .expect("thread spawn fail");
    }

    drop(tx);

    let consumer = thread::spawn(|| {
        for msg in rx {
            println!("thread-{:?}, value: {:?}", msg.ids, msg.value);
        }
    });

    consumer.join().expect("consumer thread join fail");

    println!("main thread exit");
    Ok(())
}

fn producer(thread_num: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        if value % 5 == 0 {
            println!("thread-{} exit", thread_num);
            break;
        }
        tx.send(Msg::new(thread_num, value))?;
        // 随机休眠 u8取值范围0-255 * 10 ms
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(std::time::Duration::from_millis(sleep_time));
    }
    Ok(())
}
