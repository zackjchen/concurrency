// metrics data structure
// 基本功能: inc/dec/snapshot

use anyhow::Result;
use dashmap::DashMap;
use std::{fmt::Display, sync::Arc};
#[derive(Debug, Clone, Default)]
pub struct Metrics {
    data: Arc<DashMap<String, i64>>,
}
impl Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in self.data.iter() {
            write!(f, "{}: {}   ", entry.key(), entry.value())?;
        }
        Ok(())
    }
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&mut self, key: impl Into<String>) -> Result<()> {
        let mut count = self.data.entry(key.into()).or_insert(0);
        *count += 1;
        Ok(())
    }

    pub fn dec(&mut self, key: impl Into<String>) -> Result<()> {
        // 不能直接写`?`, 这个error里面包裹了MutexGuard,导致error不能Send，需要转换error
        let mut count: dashmap::mapref::one::RefMut<String, i64, std::hash::RandomState> =
            self.data.entry(key.into()).or_insert(0);
        *count -= 1;
        Ok(())
    }
}
