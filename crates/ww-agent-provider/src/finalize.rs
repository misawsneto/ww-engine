use crate::{AssemblyError, ModelEventStream, ModelResponse, ProviderError, ResponseAssembler};
use futures_util::StreamExt;
use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum StreamFinalizationError {
    #[error(transparent)]
    Provider(#[from] ProviderError),
    #[error(transparent)]
    Assembly(#[from] AssemblyError),
}

/// Consume one normalized provider stream through the fail-closed assembler.
///
/// The stream is consumed through EOF even after a terminal event so that a
/// provider cannot hide duplicate or post-terminal output. EOF without one
/// terminal response is an assembly error.
pub async fn finalize_stream(
    mut stream: ModelEventStream,
) -> Result<ModelResponse, StreamFinalizationError> {
    let mut assembler = ResponseAssembler::new();
    while let Some(event) = stream.next().await {
        assembler.push(event?)?;
    }
    assembler.finish().map_err(Into::into)
}
