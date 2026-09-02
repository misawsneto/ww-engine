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
        let token = CancellationToken::new();
        if already_cancelled {
            token.cancel();
        }
        self.inner.lock().await.insert(id, token.clone());
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
