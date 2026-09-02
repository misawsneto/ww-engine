use crate::RuntimeError;
use sha2::{Digest, Sha256};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use ww_store::RuntimeStore;
use ww_types::{ArtifactId, ArtifactRef};

#[derive(Clone)]
pub struct LocalArtifactService {
    root: Arc<PathBuf>,
    store: Arc<dyn RuntimeStore>,
}

impl LocalArtifactService {
    pub fn new(root: impl Into<PathBuf>, store: Arc<dyn RuntimeStore>) -> Self {
        Self {
            root: Arc::new(root.into()),
            store,
        }
    }

    pub fn root(&self) -> &Path {
        self.root.as_path()
    }

    pub async fn put_bytes(
        &self,
        bytes: &[u8],
        media_type: impl Into<String>,
    ) -> Result<ArtifactRef, RuntimeError> {
        let digest = format!("{:x}", Sha256::digest(bytes));
        if let Some(existing) = self.store.get_artifact_by_digest(&digest).await? {
            return Ok(existing);
        }

        let directory = self.root.join("sha256").join(&digest[..2]);
        tokio::fs::create_dir_all(&directory).await?;
        let final_path = directory.join(&digest);
        if tokio::fs::metadata(&final_path).await.is_err() {
            let temp_path = directory.join(format!(".{digest}.{}.tmp", Uuid::now_v7()));
            let mut file = tokio::fs::File::create(&temp_path).await?;
            file.write_all(bytes).await?;
            file.sync_all().await?;
            drop(file);
            match tokio::fs::rename(&temp_path, &final_path).await {
                Ok(()) => {}
                Err(error) if tokio::fs::metadata(&final_path).await.is_ok() => {
                    let _ = tokio::fs::remove_file(&temp_path).await;
                    let _ = error;
                }
                Err(error) => return Err(error.into()),
            }
        }

        let absolute = tokio::fs::canonicalize(&final_path).await?;
        let artifact = ArtifactRef {
            id: ArtifactId::new(),
            digest,
            media_type: media_type.into(),
            size_bytes: bytes.len() as u64,
            storage_uri: format!("file://{}", absolute.display()),
        };
        self.store.put_artifact(artifact).await.map_err(Into::into)
    }
}
