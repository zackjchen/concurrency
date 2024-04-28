use anyhow::Result;
use std::{sync::mpsc, thread};
fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            producer(i, tx).unwrap();
        });
    }

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("{}: {}", msg.idx, msg.value);
        }
    });
    consumer
        .join()
        .map_err(|e| anyhow::anyhow!("error: {:?}", e))?;
    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idx, value)).unwrap();
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
#[allow(unused)]
struct Msg {
    idx: usize,
    value: usize,
}
impl Msg {
    fn new(id: usize, value: usize) -> Self {
        Self { idx: id, value }
    }
}
