use std::sync::{Arc, Mutex};
use crate::compression::chern_simons::TopologicalPacket;

/// A simple ring buffer for managing compressed topological packets
/// meant to offload straight to the 2048-bit ALUs.
pub struct BufferPool {
    pool: Arc<Mutex<Vec<TopologicalPacket>>>,
    capacity: usize,
}

impl BufferPool {
    pub fn new(capacity: usize) -> Self {
        Self {
            pool: Arc::new(Mutex::new(Vec::with_capacity(capacity))),
            capacity,
        }
    }

    pub fn push(&self, packet: TopologicalPacket) {
        let mut p = self.pool.lock().unwrap();
        if p.len() >= self.capacity {
            // Drop oldest
            p.remove(0);
        }
        p.push(packet);
    }

    pub fn pop_all(&self) -> Vec<TopologicalPacket> {
        let mut p = self.pool.lock().unwrap();
        let ret = p.clone();
        p.clear();
        ret
    }
}
