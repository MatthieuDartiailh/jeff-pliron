use std::collections::HashMap;

/// A single key/value pair attached to a JEFF operation or region.
///
/// JEFF stores metadata as semantic attributes rather than source-span text,
/// so these entries preserve the binary schema’s string-keyed payloads without
/// treating them as a textual frontend AST.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JeffMeta {
    pub name: String,
    pub value: String,
}

/// Gate-specific metadata attached to a JEFF operation.
///
/// This mirrors the semantic shape of the JEFF schema: the gate name is optional,
/// the operation may be adjoint, powers and controls are tracked explicitly, and
/// auxiliary metadata remains in a structured list rather than a flattened text blob.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GateMetadata {
    pub gate_name: Option<String>,
    pub adjoint: bool,
    pub power: Option<i32>,
    pub control_count: usize,
    pub custom: Vec<String>,
    pub metadata: Vec<JeffMeta>,
}

/// A compact string interning table used by the JEFF dialect.
///
/// The table keeps stable IDs for names and metadata keys while preserving a
/// semantic binary representation, instead of converting the program into a source
/// file or pseudo-textual IR.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct StringTable {
    pub entries: Vec<String>,
    index: HashMap<String, usize>,
}

impl StringTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn intern(&mut self, value: impl Into<String>) -> usize {
        let value = value.into();
        if let Some(&idx) = self.index.get(&value) {
            return idx;
        }
        let idx = self.entries.len();
        self.entries.push(value.clone());
        self.index.insert(value, idx);
        idx
    }

    pub fn get(&self, idx: usize) -> Option<&str> {
        self.entries.get(idx).map(String::as_str)
    }
}
