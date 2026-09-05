use serde_json::Value;
use sha2::{Digest, Sha256};

/// Deterministic bytes for one parsed value.
///
/// `serde_json::Map` is a `BTreeMap` unless the `preserve_order` feature is
/// enabled, so serialization already emits object keys in recursively sorted
/// order. No separate canonicalization pass is needed. The nested-bytes test
/// asserts those exact bytes, so enabling `preserve_order` would fail loudly
/// rather than silently change every digest.
pub fn canonical_bytes(value: &Value) -> Vec<u8> {
    serde_json::to_vec(value).expect("a parsed Value always serializes")
}

/// SHA-256 over the canonical bytes, as lowercase hex.
pub fn arguments_digest(value: &Value) -> String {
    format!("{:x}", Sha256::digest(canonical_bytes(value)))
}
