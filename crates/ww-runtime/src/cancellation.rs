use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
use ww_types::ExecutionId;

#[derive(Clone, Default)]
pub struct CancellationRegistry {
    inner: Arc<Mutex<HashMap<ExecutionId, CancellationToken>>>,
}

impl CancellationRegistry {
    pub async fn register(&self, id: ExecutionId, already_cancelled: bool) -> CancellationToken {
        let mut tokens = self.inner.lock().await;
        let token = tokens.entry(id).or_default().clone();
        if already_cancelled {
            token.cancel();
        }
        token
    }

    pub async fn signal(&self, id: ExecutionId) -> bool {
        let token = self.inner.lock().await.get(&id).cloned();
        if let Some(token) = token {
            token.cancel();
            true
        } else {
            false
        }
    }

    pub async fn unregister(&self, id: ExecutionId) {
        self.inner.lock().await.remove(&id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn repeated_registration_reuses_one_root_token() {
        let registry = CancellationRegistry::default();
        let id = ExecutionId::new();
        let first = registry.register(id, false).await;
        let second = registry.register(id, false).await;

        assert!(registry.signal(id).await);
        assert!(first.is_cancelled());
        assert!(second.is_cancelled());
    }

    #[tokio::test]
    async fn durable_cancellation_is_observed_on_later_registration() {
        let registry = CancellationRegistry::default();
        let token = registry.register(ExecutionId::new(), true).await;
        assert!(token.is_cancelled());
    }
}
