#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JeffMeta {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GateMetadata {
    pub gate_name: Option<String>,
    pub adjoint: bool,
    pub power: Option<i32>,
    pub control_count: usize,
    pub custom: Vec<String>,
    pub metadata: Vec<JeffMeta>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StringTable {
    pub entries: Vec<String>,
}

impl StringTable {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn intern(&mut self, value: impl Into<String>) -> usize {
        let value = value.into();
        if let Some(idx) = self.entries.iter().position(|existing| existing == &value) {
            return idx;
        }
        self.entries.push(value);
        self.entries.len() - 1
    }

    pub fn get(&self, idx: usize) -> Option<&str> {
        self.entries.get(idx).map(String::as_str)
    }
}
