pub mod types;
pub mod message_service;
pub mod router;
pub mod diagnostics;

#[cfg(test)]
mod tests {
    use tokio::task;
    use crate::message_service::MessageService;
    use crate::router::TxRouter;
    use crate::types::{Message, Transaction};

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
        let file_manager = FileProcessor::new();

        let mut tasks = vec![];

        for file_id in 1..50 {
            let manager_clone = file_manager.clone();

            tasks.push(tokio::spawn(async move {
                manager_clone.process_file(File {
                    id: file_id,
                    filename: format!("file_{}.txt", file_id),
                    content: vec![],
                }).await;
            }));
        }

        for task in tasks {
            task.await.unwrap();
        }

        assert_eq!(file_manager.processed_files_count().await, 50);
    }
    
}