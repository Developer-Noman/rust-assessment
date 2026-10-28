use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::types::{Batch, Transaction};

#[derive(Clone)]
pub struct TxRouter {
    pending: Arc<Mutex<Vec<Transaction>>>,
    seen: Arc<Mutex<HashSet<u64>>>,
    failed: Arc<Mutex<HashMap<u64, u64>>>,
    batch_counter: Arc<Mutex<u64>>,
}

impl TxRouter {
    pub fn new() -> Self {
        Self {
            pending: Arc::new(Mutex::new(vec![])),
            seen: Arc::new(Mutex::new(HashSet::new())),
            failed: Arc::new(Mutex::new(HashMap::new())),
            batch_counter: Arc::new(Mutex::new(1)),
        }
    }

    // -------------------------
    // Handles incoming transaction submissions
    // -------------------------
    pub async fn submit(&self, tx: Transaction) {
        let mut pending = self.pending.lock().await;
        let seen = self.seen.lock().await;

        if seen.contains(&tx.id) || pending.iter().any(|waiting| waiting.id == tx.id) {
            return;
        }

        pending.push(tx);
    }


    pub async fn build_batch(&self, limit: usize) -> Option<Batch> {
        let mut pending = self.pending.lock().await;

        if pending.is_empty() || limit == 0 {
            return None;
        }

        let take = limit.min(pending.len());
        let items = pending.drain(0..take).collect::<Vec<_>>();

        let mut counter = self.batch_counter.lock().await;

        let batch = Batch {
            batch_id: *counter,
            transactions: items,
        };

        *counter += 1;

        Some(batch)
    }

 
    pub async fn mark_failed(&self, tx_id: u64) {
        let mut failed = self.failed.lock().await;

        failed.insert(tx_id, 1);
    }

 
    pub async fn finalize_batch(&self, batch: Batch) {
        let mut seen = self.seen.lock().await;

        for tx in batch.transactions {
            seen.insert(tx.id);
        }
    }

    pub async fn pending_count(&self) -> usize {
        let pending = self.pending.lock().await;
        pending.len()
    }

    pub async fn failed_count(&self) -> usize {
        let failed = self.failed.lock().await;
        failed.len()
    }
}