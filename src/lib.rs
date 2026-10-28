pub mod types;
pub mod message_service;
pub mod router;
pub mod diagnostics;

#[cfg(test)]
mod tests {
    use crate::router::TxRouter;
    use crate::types::Transaction;

    #[tokio::test]
    async fn test_submit_and_count() {
        let router = TxRouter::new();

        router.submit(Transaction {
            id: 1,
            wallet: "alice".into(),
            value: 100,
        }).await;

        assert_eq!(router.pending_count().await, 1);
    }

    #[tokio::test]
    async fn test_batch_creation() {
        let router = TxRouter::new();

        for i in 0..5 {
            router.submit(Transaction {
                id: i,
                wallet: "user".into(),
                value: 10,
            }).await;
        }

        let batch = router.build_batch(3).await.unwrap();

        assert_eq!(batch.transactions.len(), 3);
        assert_eq!(router.pending_count().await, 2);
    }
    
    #[tokio::test]
    async fn test_finalize_marks_seen() {
        let router = TxRouter::new();

        router.submit(Transaction {
            id: 10,
            wallet: "alice".into(),
            value: 100,
        }).await;

        let batch = router.build_batch(10).await.unwrap();

        router.finalize_batch(batch).await;

        router.submit(Transaction {
            id: 10,
            wallet: "alice".into(),
            value: 100,
        }).await;

        assert_eq!(router.pending_count().await, 0);
    }

    #[tokio::test]
    async fn test_failed_tracking() {
        let router = TxRouter::new();

        router.mark_failed(99).await;

        assert_eq!(router.failed_count().await, 1);
    }

    #[tokio::test]
    async fn test_concurrent_submissions() {
        let router = TxRouter::new();

        let mut tasks = vec![];

        for tx_id in 1..=50 {
            let router = router.clone();

            tasks.push(tokio::spawn(async move {
                router.submit(Transaction {
                    id: tx_id,
                    wallet: format!("wallet_{tx_id}"),
                    value: 1,
                }).await;
            }));
        }

        for task in tasks {
            task.await.unwrap();
        }

        assert_eq!(router.pending_count().await, 50);
    }

    #[tokio::test]
    async fn test_build_batch_limit_larger_than_queue() {
        let router = TxRouter::new();

        for id in 0..2 {
            router.submit(Transaction {
                id,
                wallet: "user".into(),
                value: 10,
            }).await;
        }

        let batch = router.build_batch(10).await.unwrap();

        assert_eq!(batch.transactions.len(), 2);
        assert_eq!(router.pending_count().await, 0);
    }

    #[tokio::test]
    async fn test_build_batch_limit_zero() {
        let router = TxRouter::new();

        router.submit(Transaction {
            id: 1,
            wallet: "alice".into(),
            value: 100,
        }).await;

        assert!(router.build_batch(0).await.is_none());
        assert_eq!(router.pending_count().await, 1);
    }

    #[tokio::test]
    async fn test_duplicate_id_is_ignored() {
        let router = TxRouter::new();

        let tx = Transaction {
            id: 7,
            wallet: "alice".into(),
            value: 100,
        };

        router.submit(tx.clone()).await;
        router.submit(tx).await;

        assert_eq!(router.pending_count().await, 1);
    }

    #[tokio::test]
    async fn test_resubmit_after_finalize() {
        let router = TxRouter::new();

        router.submit(Transaction {
            id: 10,
            wallet: "alice".into(),
            value: 100,
        }).await;

        let batch = router.build_batch(1).await.unwrap();
        router.finalize_batch(batch).await;

        router.submit(Transaction {
            id: 10,
            wallet: "alice".into(),
            value: 100,
        }).await;

        assert_eq!(router.pending_count().await, 0);

        router.submit(Transaction {
            id: 11,
            wallet: "bob".into(),
            value: 50,
        }).await;

        assert_eq!(router.pending_count().await, 1);
    }
}