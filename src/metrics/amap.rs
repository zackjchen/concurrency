use dashmap::DashMap;
use std::sync::atomic::Ordering;
use std::{
    fmt::Display,
    sync::{atomic::AtomicI64, Arc},
};

#[derive(Debug)]
pub struct AmapMetrics {
    data: Arc<DashMap<&'static str, AtomicI64>>,
}

impl Display for AmapMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in self.data.iter() {
            write!(
                f,
                "{}: {}   ",
                entry.key(),
                entry.value().load(Ordering::Relaxed)
            )?;
        }
        Ok(())
    }
}

impl Clone for AmapMetrics {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl AmapMetrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        let data = Arc::new(DashMap::new());
        for name in metric_names {
            data.insert(*name, AtomicI64::new(0));
        }
        Self { data }
    }

    pub fn inc(&self, key: &str) {
        if let Some(value) = self.data.get(key) {
            value.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn dec(&self, key: &str) {
        if let Some(value) = self.data.get(key) {
            value.fetch_sub(1, Ordering::Relaxed);
        }
    }
}
