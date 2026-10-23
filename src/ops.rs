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
