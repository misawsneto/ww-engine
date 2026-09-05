use crate::{
    CompiledSchema, Tool, ToolDefinitionError, ToolId, ToolIdentity, ToolResolutionError, ToolSpec,
};
use std::{collections::BTreeMap, fmt, sync::Arc};

/// One registered tool with its compiled validator.
pub struct RegisteredTool {
    tool: Arc<dyn Tool>,
    spec: ToolSpec,
    schema: CompiledSchema,
}

// `Arc<dyn Tool>` is not `Debug`, so report the pin instead of the executor.
impl fmt::Debug for RegisteredTool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RegisteredTool")
            .field("identity", &self.spec.identity)
            .finish_non_exhaustive()
    }
}

impl RegisteredTool {
    pub fn spec(&self) -> &ToolSpec {
        &self.spec
    }

    pub fn schema(&self) -> &CompiledSchema {
        &self.schema
    }

    /// The executor the preparation seam retains for an allowed call.
    pub fn tool(&self) -> &Arc<dyn Tool> {
        &self.tool
    }
}

/// The tools available to one Agent run.
///
/// The registry is immutable for the run and holds no run state. Registration
/// order carries no model-visible authority; only the run's configured pin
/// order does.
pub struct ToolRegistry {
    entries: BTreeMap<ToolId, RegisteredTool>,
}

impl fmt::Debug for ToolRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ToolRegistry")
            .field("registered", &self.entries.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl ToolRegistry {
    pub fn build(tools: Vec<Arc<dyn Tool>>) -> Result<Self, ToolDefinitionError> {
        let mut entries: BTreeMap<ToolId, RegisteredTool> = BTreeMap::new();
        for tool in tools {
            let spec = tool.spec();
            let id = spec.identity.id.clone();
            if entries.contains_key(&id) {
                return Err(ToolDefinitionError::DuplicateId { id });
            }
            let schema = CompiledSchema::compile(&spec.input_schema)?;
            entries.insert(id, RegisteredTool { tool, spec, schema });
        }
        Ok(Self { entries })
    }

    /// Resolve one exact pin. A version mismatch never substitutes.
    pub fn resolve(&self, identity: &ToolIdentity) -> Result<&RegisteredTool, ToolResolutionError> {
        let entry =
            self.entries
                .get(&identity.id)
                .ok_or_else(|| ToolResolutionError::NotFound {
                    id: identity.id.clone(),
                })?;
        if entry.spec.identity.version != identity.version {
            return Err(ToolResolutionError::VersionMismatch {
                id: identity.id.clone(),
                requested: identity.version.as_str().to_owned(),
                available: entry.spec.identity.version.as_str().to_owned(),
            });
        }
        Ok(entry)
    }

    /// Project the model-visible specs for the run's configured pins.
    ///
    /// The result contains exactly those pins, in configured order.
    pub fn project(&self, pins: &[ToolIdentity]) -> Result<Vec<&ToolSpec>, ToolResolutionError> {
        pins.iter()
            .map(|pin| self.resolve(pin).map(RegisteredTool::spec))
            .collect()
    }
}
