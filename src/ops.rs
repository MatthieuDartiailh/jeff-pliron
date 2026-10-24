use crate::{attributes::StringTable, types::JeffType};
use pliron::derive::pliron_op;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JeffOpKind {
    QubitAlloc,
    QubitFree,
    QubitFreeZero,
    QubitMeasure,
    QubitMeasureNd,
    QubitReset,
    GPhase,
    I,
    X,
    Y,
    Z,
    S,
    T,
    R1,
    Rx,
    Ry,
    Rz,
    H,
    U,
    Swap,
    Ppr,
    Custom,
    QuregAlloc,
    QuregFree,
    QuregFreeZero,
    QuregExtractIndex,
    QuregInsertIndex,
    QuregExtractSlice,
    QuregInsertSlice,
    QuregLength,
    QuregSplit,
    QuregJoin,
    QuregCreate,
    IntConst,
    IntAdd,
    IntSub,
    IntMul,
    IntDiv,
    IntCmp,
    IntBitAnd,
    IntBitOr,
    IntBitXor,
    IntSelect,
    FloatConst,
    FloatAdd,
    FloatMul,
    FloatCmp,
    FuncCall,
    ScfSwitch,
    ScfFor,
    ScfWhile,
}

impl JeffOpKind {
    pub fn semantic_name(&self) -> &'static str {
        match self {
            Self::QubitAlloc => "jeff.qubit_alloc",
            Self::QubitFree => "jeff.qubit_free",
            Self::QubitFreeZero => "jeff.qubit_free_zero",
            Self::QubitMeasure => "jeff.qubit_measure",
            Self::QubitMeasureNd => "jeff.qubit_measure_nd",
            Self::QubitReset => "jeff.qubit_reset",
            Self::GPhase => "jeff.gphase",
            Self::I => "jeff.i",
            Self::X => "jeff.x",
            Self::Y => "jeff.y",
            Self::Z => "jeff.z",
            Self::S => "jeff.s",
            Self::T => "jeff.t",
            Self::R1 => "jeff.r1",
            Self::Rx => "jeff.rx",
            Self::Ry => "jeff.ry",
            Self::Rz => "jeff.rz",
            Self::H => "jeff.h",
            Self::U => "jeff.u",
            Self::Swap => "jeff.swap",
            Self::Ppr => "jeff.ppr",
            Self::Custom => "jeff.custom",
            Self::QuregAlloc => "jeff.qureg_alloc",
            Self::QuregFree => "jeff.qureg_free",
            Self::QuregFreeZero => "jeff.qureg_free_zero",
            Self::QuregExtractIndex => "jeff.qureg_extract_index",
            Self::QuregInsertIndex => "jeff.qureg_insert_index",
            Self::QuregExtractSlice => "jeff.qureg_extract_slice",
            Self::QuregInsertSlice => "jeff.qureg_insert_slice",
            Self::QuregLength => "jeff.qureg_length",
            Self::QuregSplit => "jeff.qureg_split",
            Self::QuregJoin => "jeff.qureg_join",
            Self::QuregCreate => "jeff.qureg_create",
            Self::IntConst => "jeff.int_const",
            Self::IntAdd => "jeff.int_add",
            Self::IntSub => "jeff.int_sub",
            Self::IntMul => "jeff.int_mul",
            Self::IntDiv => "jeff.int_div",
            Self::IntCmp => "jeff.int_cmp",
            Self::IntBitAnd => "jeff.int_bitand",
            Self::IntBitOr => "jeff.int_bitor",
            Self::IntBitXor => "jeff.int_bitxor",
            Self::IntSelect => "jeff.int_select",
            Self::FloatConst => "jeff.float_const",
            Self::FloatAdd => "jeff.float_add",
            Self::FloatMul => "jeff.float_mul",
            Self::FloatCmp => "jeff.float_cmp",
            Self::FuncCall => "jeff.func_call",
            Self::ScfSwitch => "jeff.scf_switch",
            Self::ScfFor => "jeff.scf_for",
            Self::ScfWhile => "jeff.scf_while",
        }
    }
}

#[pliron_op(name = "jeff.qubit_alloc", format, verifier = "succ")]
pub struct QubitAllocOp;

#[pliron_op(name = "jeff.qubit_free", format, verifier = "succ")]
pub struct QubitFreeOp;

#[pliron_op(name = "jeff.qubit_measure", format, verifier = "succ")]
pub struct QubitMeasureOp;

#[pliron_op(name = "jeff.qubit_gate", format, verifier = "succ")]
pub struct QubitGateOp;

#[pliron_op(name = "jeff.qureg_alloc", format, verifier = "succ")]
pub struct QuregAllocOp;

#[pliron_op(name = "jeff.qureg_length", format, verifier = "succ")]
pub struct QuregLengthOp;

#[pliron_op(name = "jeff.scf_switch", format, verifier = "succ")]
pub struct ScfSwitchOp;

#[pliron_op(name = "jeff.scf_for", format, verifier = "succ")]
pub struct ScfForOp;

#[pliron_op(name = "jeff.scf_while", format, verifier = "succ")]
pub struct ScfWhileOp;

#[pliron_op(name = "jeff.func_call", format, verifier = "succ")]
pub struct FuncCallOp;

#[pliron_op(name = "jeff.func_decl", format, verifier = "succ")]
pub struct FuncDeclOp;

#[pliron_op(name = "jeff.func_def", format, verifier = "succ")]
pub struct FuncDefOp;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JeffValue {
    pub ty: JeffType,
    pub name: Option<String>,
    pub index: Option<usize>,
}

impl JeffValue {
    pub fn new(ty: JeffType) -> Self {
        Self {
            ty,
            name: None,
            index: None,
        }
    }

    pub fn named(ty: JeffType, name: impl Into<String>) -> Self {
        Self {
            ty,
            name: Some(name.into()),
            index: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JeffOp {
    pub kind: JeffOpKind,
    pub name: Option<String>,
    pub inputs: Vec<JeffValue>,
    pub outputs: Vec<JeffValue>,
    pub metadata: Vec<crate::attributes::JeffMeta>,
}

impl JeffOp {
    pub fn new(kind: JeffOpKind) -> Self {
        Self {
            kind,
            name: None,
            inputs: Vec::new(),
            outputs: Vec::new(),
            metadata: Vec::new(),
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_metadata(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.push(crate::attributes::JeffMeta {
            name: name.into(),
            value: value.into(),
        });
        self
    }

    pub fn semantic_name(&self) -> &'static str {
        self.kind.semantic_name()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct JeffRegion {
    pub name: Option<String>,
    pub ops: Vec<JeffOp>,
    pub metadata: Vec<crate::attributes::JeffMeta>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct JeffFunction {
    pub name: Option<String>,
    pub arguments: Vec<JeffValue>,
    pub results: Vec<JeffValue>,
    pub regions: Vec<JeffRegion>,
    pub metadata: Vec<crate::attributes::JeffMeta>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JeffModule {
    pub name: Option<String>,
    pub strings: StringTable,
    pub functions: Vec<JeffFunction>,
    pub metadata: Vec<crate::attributes::JeffMeta>,
}

impl Default for JeffModule {
    fn default() -> Self {
        Self {
            name: None,
            strings: StringTable::new(),
            functions: Vec::new(),
            metadata: Vec::new(),
        }
    }
}

impl JeffModule {
    pub fn with_function(mut self, function: JeffFunction) -> Self {
        self.functions.push(function);
        self
    }

    pub fn with_metadata(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.push(crate::attributes::JeffMeta {
            name: name.into(),
            value: value.into(),
        });
        self
    }
}

impl JeffFunction {
    pub fn with_region(mut self, region: JeffRegion) -> Self {
        self.regions.push(region);
        self
    }

    pub fn with_metadata(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.push(crate::attributes::JeffMeta {
            name: name.into(),
            value: value.into(),
        });
        self
    }
}

impl JeffRegion {
    pub fn with_op(mut self, op: JeffOp) -> Self {
        self.ops.push(op);
        self
    }

    pub fn with_metadata(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.push(crate::attributes::JeffMeta {
            name: name.into(),
            value: value.into(),
        });
        self
    }
}
