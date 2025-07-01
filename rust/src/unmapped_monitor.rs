use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Serialize};

#[derive(Debug, Default, Clone, Serialize)]
pub struct UnmappedEntry {
    pub count: usize,
    pub samples: Vec<String>,
}

pub struct UnmappedFieldMonitor {
    map: Mutex<HashMap<String, UnmappedEntry>>,
}

impl UnmappedFieldMonitor {
    pub fn log(&self, name: &str, value: Option<f64>) {
        let mut guard = self.map.lock().unwrap();
        let entry = guard.entry(name.to_ascii_lowercase()).or_default();
        entry.count += 1;
        if let Some(v) = value {
            if entry.samples.len() < 3 {
                entry.samples.push(format!("{v}"));
            }
        }
    }

    pub fn snapshot(&self) -> HashMap<String, UnmappedEntry> {
        self.map.lock().unwrap().clone()
    }

    pub fn reset(&self) {
        self.map.lock().unwrap().clear();
    }
}

pub static UNMAPPED_MONITOR: Lazy<UnmappedFieldMonitor> = Lazy::new(|| {
    UnmappedFieldMonitor {
        map: Mutex::new(HashMap::new()),
    }
});
