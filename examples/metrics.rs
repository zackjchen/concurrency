use std::thread;

use anyhow::Result;
use concurrency::metrics::Metrics;
use rand::Rng as _;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = Metrics::new();

    // start n workers and M requesters
    for idx in 0..N {
        task_worker(idx, metrics.clone())?;
    }
    for _ in 0..M {
        request_worker(metrics.clone())?;
    }
    loop {
        thread::sleep(std::time::Duration::from_secs(1));
        println!("{}", metrics);
    }
    // Ok(())
}

fn task_worker(index: usize, mut metrics: Metrics) -> Result<()> {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(std::time::Duration::from_millis(rng.gen_range(100..5000)));
        metrics
            .inc(format!("call.thread.worker.{:?}", index))
            .unwrap();
    });
    Ok(())
}

fn request_worker(mut metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(std::time::Duration::from_millis(rng.gen_range(100..5000)));
            metrics.inc("req.page.1")?;
            metrics.inc("req.page.2")?;
            metrics.inc("req.page.3")?;
        }

        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
